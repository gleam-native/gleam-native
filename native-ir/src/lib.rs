// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The serializable intermediate representation for the Gleam native target.
//!
//! `compiler-core` lowers typed Gleam modules into this IR at build time and
//! writes one artifact file per module; `native-generation` loads the
//! artifacts and translates them to Cranelift IR. This crate must stay free
//! of Cranelift so that `compiler-core` remains compilable to WebAssembly.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Bumped whenever the types in this crate change shape, so that stale
/// artifacts from previous compiler builds are rejected rather than
/// misinterpreted. bitcode is not a self-describing format.
pub const FORMAT_VERSION: u32 = 33;

/// Whether the bytes are an artifact of the current format version, from
/// the four-byte little-endian version header alone. The build uses this to
/// treat modules with outdated artifacts as stale, so a compiler upgrade
/// regenerates them instead of failing at run time.
pub fn artifact_is_current(bytes: &[u8]) -> bool {
    bytes.len() >= 4
        && u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) == FORMAT_VERSION
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// The Gleam module name, with `/` separators, e.g. `gleam/wibble`.
    pub name: String,
    /// The module's source path relative to its package root, e.g.
    /// `src/main.gleam`, printed by `echo` like the other targets do.
    pub src_path: String,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Function {
    /// A function with a Gleam body, compiled to native code.
    Defined {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    /// A function implemented by `@external(native, _, symbol)`. Calls
    /// resolve to `symbol` with the platform C calling convention.
    External {
        name: String,
        arity: u32,
        symbol: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
        /// The 1-based source line, or zero for synthesized statements;
        /// carried into the generated code's debug line information.
        line: u32,
    },
    /// A `let` with a non-trivial pattern, or a `let assert`. The subject is
    /// bound as decision variable `subject_id` and the tree's bindings
    /// persist in the enclosing scope. `Fail` panics with `on_failure` for
    /// `let assert`, and is unreachable (a trap) for irrefutable `let`.
    Destructure {
        subject: Expression,
        subject_id: u32,
        tree: Decision,
        on_failure: Option<AssignmentFailure>,
        /// The source line, zero when synthesized; see [`Statement::Let`].
        line: u32,
    },
    Expression {
        expression: Expression,
        /// The source line, zero when synthesized; see [`Statement::Let`].
        line: u32,
    },
}

impl Statement {
    /// An expression statement with no source line: for statements the
    /// lowering synthesizes rather than reads from source.
    pub fn expression(expression: Expression) -> Statement {
        Statement::Expression {
            expression,
            line: 0,
        }
    }

    /// The statement's source line, zero when synthesized.
    pub fn line(&self) -> u32 {
        match self {
            Statement::Let { line, .. }
            | Statement::Destructure { line, .. }
            | Statement::Expression { line, .. } => *line,
        }
    }
}

/// How a failed `let assert` reports itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignmentFailure {
    pub message: Option<Box<Expression>>,
    pub function: String,
    pub line: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// An integer literal small enough to be stored as a tagged immediate.
    Int(i64),
    /// An integer literal too large for a tagged immediate: the value as
    /// signed little-endian bytes, built into a heap big integer at run time.
    BigInt(Vec<u8>),
    /// A float literal, boxed on the heap at run time.
    Float(f64),
    /// A string literal with escape sequences already processed, built into
    /// an immutable UTF-8 heap string at run time.
    String(String),
    Nil,
    /// `True` or `False`. Like `Nil` and the empty list, booleans are
    /// special immediate words distinct from every integer; code
    /// generation owns the exact encodings.
    Bool(bool),
    Variable(String),
    Block(Vec<Statement>),
    Call {
        module: String,
        function: String,
        arguments: Vec<Expression>,
    },
    /// Integer arithmetic with a small-integer fast path and overflow
    /// promotion to heap big integers.
    IntBinary {
        operator: IntOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Integer ordering comparison, yielding a boolean.
    IntCompare {
        operator: CompareOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Float arithmetic on boxed f64 values.
    FloatBinary {
        operator: FloatOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Float ordering comparison, yielding a boolean. IEEE ordered
    /// semantics: comparisons involving NaN are false.
    FloatCompare {
        operator: CompareOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// `==` or `!=`. Both operands share one static Gleam type, so the
    /// comparison strategy is chosen at compile time.
    Equality {
        kind: EqualityKind,
        negated: bool,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Boolean negation (guards only, so far).
    BoolNot(Box<Expression>),
    /// `&&` or `||` with short-circuit evaluation: the right expression is
    /// evaluated only when the left one does not decide the result.
    BoolBinary {
        operator: BoolOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    StringConcat(Box<Expression>, Box<Expression>),
    /// A `case` expression, lowered from the compiler's exhaustiveness
    /// decision tree. Subjects are evaluated once, in order, then the tree
    /// decides which clause body runs.
    ///
    /// Decision variables are numbered: `subject_ids[i]` is the variable
    /// holding subject `i`, and variant checks introduce further variables
    /// for the fields they extract.
    Case {
        subjects: Vec<Expression>,
        subject_ids: Vec<u32>,
        tree: Decision,
    },
    /// Constructing a custom type value: a heap record with a variant tag
    /// word followed by the field values. The display says how `echo`
    /// renders the value.
    Constructor {
        tag: u32,
        display: ConstructorDisplay,
        arguments: Vec<Expression>,
    },
    /// Reading field `index` out of a custom type record or tuple.
    FieldAccess {
        record: Box<Expression>,
        index: u32,
    },
    /// The empty list, a special immediate word. Cons cells are two-field records
    /// with tag 1, built with [`Expression::List`] (or, equivalently, an
    /// [`Expression::Constructor`] with tag 1).
    EmptyList,
    /// A list literal, kept flat: a nested cons chain would make the
    /// expression tree — and every recursive walk over it, serialization
    /// included — as deep as the list is long. Elements evaluate left to
    /// right, then the tail (the empty list when absent), then the cells
    /// are built from the tail outwards.
    List {
        elements: Vec<Expression>,
        tail: Option<Box<Expression>>,
    },
    /// Bit array construction: segments appended in order onto an empty
    /// array. Only byte-aligned, constant-sized segments are supported.
    BitArray(Vec<BitSegment>),
    /// An anonymous function. Lambda-lifted at code generation time: its
    /// free variables become the captures of a heap closure
    /// `[code pointer, captures...]`.
    Lambda {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    /// A module function (or external) used as a value: becomes a closure
    /// around a generated wrapper.
    FunctionReference {
        module: String,
        function: String,
        arity: u32,
    },
    /// Calling a function value: load the code pointer out of the closure
    /// and call it with the closure as the first argument.
    CallValue {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    /// An `echo` expression: prints the source location and the value to
    /// standard error and evaluates to the value. Rendering is structural:
    /// every value identifies itself at run time.
    Echo {
        value: Box<Expression>,
        message: Option<Box<Expression>>,
        line: u32,
    },
    /// A `panic` or `todo` expression: prints an error naming the source
    /// location and aborts the program. The message, when present, is a
    /// string expression evaluated only if the panic is reached.
    Panic {
        kind: PanicKind,
        message: Option<Box<Expression>>,
        function: String,
        line: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PanicKind {
    Panic,
    Todo,
    LetAssert,
    Assert,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CompareOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BoolOperator {
    And,
    Or,
}

/// One segment of a bit array construction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitSegment {
    pub value: Box<Expression>,
    pub kind: BitSegmentKind,
}

/// Byte order of a multi-byte segment. `Native` resolves to the host's
/// byte order at run time.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Endian {
    Big,
    Little,
    Native,
}

/// A string segment's encoding.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StringEncoding {
    Utf8,
    Utf16,
    Utf32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BitSegmentKind {
    /// An integer segment, truncated to the segment size. The size is an
    /// expression evaluating to the bit count.
    Int {
        bits: Box<Expression>,
        endian: Endian,
    },
    /// A float segment of 16, 32 or 64 bits (validated at run time).
    Float {
        bits: Box<Expression>,
        endian: Endian,
    },
    /// A string's bytes in the given encoding.
    String {
        encoding: StringEncoding,
        endian: Endian,
    },
    /// A single UTF codepoint (a scalar value) in the given encoding.
    Codepoint {
        encoding: StringEncoding,
        endian: Endian,
    },
    /// Another bit array spliced in, whole or its first `bits` bits.
    BitArraySplice { bits: Option<Box<Expression>> },
}

/// A named segment read a bit array check materializes before evaluating:
/// later segment sizes may refer to earlier segments (`<<len,
/// payload:size(len)>>`). The names use a `bit$` prefix, which cannot
/// collide with Gleam identifiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentRead {
    pub name: String,
    pub offset: Box<Expression>,
    pub bits: Box<Expression>,
    pub endian: Endian,
    pub signed: bool,
}

/// A test a bit array check performs, with offsets and sizes as tagged
/// integer expressions (possibly referring to materialized segment reads).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BitsTest {
    /// The array is exactly (or at least) this many bits long.
    Size { bits: Box<Expression>, exact: bool },
    /// A computed size is not negative.
    NonNegative { value: Box<Expression> },
    /// The `bit_length` bits at the offset equal these constant bits
    /// (MSB-first packed).
    Bytes {
        offset: Box<Expression>,
        bytes: Vec<u8>,
        bit_length: u64,
    },
    /// The float at the given position is finite; float segments only
    /// match finite values.
    IsFiniteFloat {
        offset: Box<Expression>,
        bits: Box<Expression>,
        endian: Endian,
    },
    /// The integer at the given position (read with the segment's
    /// endianness and signedness) equals the literal, compared
    /// numerically; used when the segment's size is only known at run
    /// time (constant-size literal integers use [`BitsTest::Bytes`]).
    IntEquals {
        offset: Box<Expression>,
        bits: Box<Expression>,
        endian: Endian,
        signed: bool,
        /// The literal, an `Int` or `BigInt` expression.
        value: Box<Expression>,
    },
    /// The float at the given position equals the literal, compared
    /// numerically (like the other targets: NaN data never matches, and
    /// negative zero equals zero). Sizes other than 16, 32, and 64 bits
    /// never match.
    FloatEquals {
        offset: Box<Expression>,
        bits: Box<Expression>,
        endian: Endian,
        value: f64,
    },
    /// The remainder past the offset is a whole number of bytes.
    RestIsBytes { offset: Box<Expression> },
    /// Nothing to test (variable and discard segment patterns); the check
    /// exists to materialize its reads.
    AlwaysTrue,
}

/// How a constructed record renders in `echo` output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstructorDisplay {
    Tuple,
    List,
    Record { name: String },
}

/// A node of a lowered pattern-match decision tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Decision {
    /// A pattern matched: bind its variables, then run the clause body.
    Run {
        bindings: Vec<(String, Bound)>,
        body: Vec<Statement>,
    },
    /// Try each check against the variable in order; on the first success
    /// follow its decision, otherwise the fallback. The type system
    /// guarantees the fallback always matches; when the fallback is the
    /// final variant of an exhaustive match, `fallback_fields` are the
    /// decision variables for that variant's fields, extracted without any
    /// tag test.
    Switch {
        var: u32,
        choices: Vec<(Check, Decision)>,
        fallback: Box<Decision>,
        fallback_fields: Vec<u32>,
    },
    /// A clause guard: bind the pattern's variables, evaluate the guard
    /// expression, and run the clause body when it is true; otherwise
    /// continue with `if_false` (where the bindings are not in scope).
    Guard {
        bindings: Vec<(String, Bound)>,
        guard: Box<Expression>,
        if_true: Vec<Statement>,
        if_false: Box<Decision>,
    },
    /// Unreachable for exhaustive matches; traps if ever executed.
    Fail,
}

/// The value bound to a pattern variable when a clause matches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Bound {
    /// A decision variable: a subject or an extracted field.
    Variable(u32),
    /// A literal from the pattern itself.
    Value(Expression),
    /// The rest of a string prefix pattern: a fresh string holding the
    /// subject's contents from the given byte offset onwards.
    StringSlice { subject: u32, offset: u32 },
    /// An integer read out of a bit array segment.
    BitsReadInt {
        subject: u32,
        offset: Box<Expression>,
        bits: Box<Expression>,
        endian: Endian,
        signed: bool,
    },
    /// A float read out of a bit array segment (16, 32 or 64 bits).
    BitsReadFloat {
        subject: u32,
        offset: Box<Expression>,
        bits: Box<Expression>,
        endian: Endian,
    },
    /// A fresh bit array sliced out of another; `bits` of `None` means
    /// everything from the offset onwards.
    BitsSlice {
        subject: u32,
        offset: Box<Expression>,
        bits: Option<Box<Expression>>,
    },
}

/// A runtime check a decision tree performs against a subject.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Check {
    /// The subject is a small integer equal to this tagged-encodable value.
    Int(i64),
    /// The subject equals this big integer (signed little-endian bytes).
    BigInt(Vec<u8>),
    /// The subject is a float equal to this value.
    Float(f64),
    /// The subject is a string with these contents.
    String(String),
    /// The subject is this boolean's immediate word.
    Bool(bool),
    /// The subject is `Nil`'s immediate word (always matches; `Nil` has
    /// one variant).
    Nil,
    /// The subject is the empty list's immediate word.
    EmptyList,
    /// The subject is a custom type record with this variant tag. On a
    /// match, the record's fields become the given decision variables.
    Variant { tag: u32, fields: Vec<u32> },
    /// Always matches (tuples): the fields become decision variables.
    Always { fields: Vec<u32> },
    /// The subject is a non-empty list: its head and tail become decision
    /// variables.
    NonEmptyList { first: u32, rest: u32 },
    /// The subject is a string starting with these contents (escape
    /// sequences already processed). The rest is bound separately via
    /// [`Bound::StringSlice`].
    StringPrefix { prefix: String },
    /// A bit array test: first the segment reads are materialized as named
    /// values, then the test runs.
    BitArray {
        reads: Vec<SegmentRead>,
        test: BitsTest,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EqualityKind {
    /// Small integers compare directly; big integers via the runtime.
    Int,
    /// Boxed floats compare by IEEE `fcmp` on their loaded values.
    Float,
    /// Strings compare by contents via the runtime.
    String,
    /// Values that are always immediate words (`Bool`, `Nil`) compare as
    /// plain words.
    Immediate,
    /// Any other type: structural deep equality in the runtime, walking
    /// heap object headers.
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FloatOperator {
    Add,
    Subtract,
    Multiply,
    /// Division by zero yields 0.0.
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IntOperator {
    Add,
    Subtract,
    Multiply,
    /// Truncating division; division by zero yields zero.
    Divide,
    /// Remainder with the sign of the dividend; a zero divisor yields zero.
    Remainder,
}

/// Encodes a module as a four-byte little-endian [`FORMAT_VERSION`] header
/// followed by the serialized module.
pub fn encode(module: &Module) -> Result<Vec<u8>, bitcode::Error> {
    let mut bytes = FORMAT_VERSION.to_le_bytes().to_vec();
    bytes.extend_from_slice(&bitcode::serialize(module)?);
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<Module, String> {
    if !artifact_is_current(bytes) {
        return Err("outdated or corrupt native artifact. \
Delete the project's `build` directory and rebuild."
            .into());
    }
    bitcode::deserialize(&bytes[4..]).map_err(|error| {
        format!(
            "corrupt native artifact ({error}). \
Delete the project's `build` directory and rebuild."
        )
    })
}

/// The free variables of a statement sequence, in deterministic order:
/// variables referenced but not bound within it. Used to compute lambda
/// captures. Over-approximation is harmless (captured values are immutable);
/// a missed variable fails loudly at code generation as an unbound variable.
pub fn free_variables(statements: &[Statement], bound: &HashSet<String>) -> BTreeSet<String> {
    let mut free = BTreeSet::new();
    let mut bound = bound.clone();
    statements_free(statements, &mut bound, &mut |name| {
        let _ = free.insert(name.to_string());
    });
    free
}

/// How many times each free variable is mentioned across the statements,
/// walking exactly like [`free_variables`]. Code generation's last-use
/// analysis consumes a binding at a use only when its count says that use
/// is the only one left.
pub fn mention_counts(statements: &[Statement]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    let mut bound = HashSet::new();
    statements_free(statements, &mut bound, &mut |name| {
        *counts.entry(name.to_string()).or_insert(0) += 1;
    });
    counts
}

/// Adds one expression's free-variable mention counts into `counts`.
pub fn expression_mentions(expression: &Expression, counts: &mut HashMap<String, usize>) {
    let mut bound = HashSet::new();
    expression_free(expression, &mut bound, &mut |name| {
        *counts.entry(name.to_string()).or_insert(0) += 1;
    });
}

/// Adds a decision tree's free-variable mention counts into `counts`.
pub fn decision_mentions(decision: &Decision, counts: &mut HashMap<String, usize>) {
    let mut bound = HashSet::new();
    decision_free(
        decision,
        &mut bound,
        &mut |name| {
            *counts.entry(name.to_string()).or_insert(0) += 1;
        },
        false,
    );
}

fn statements_free(
    statements: &[Statement],
    bound: &mut HashSet<String>,
    sink: &mut dyn FnMut(&str),
) {
    for statement in statements {
        match statement {
            Statement::Let { name, value, .. } => {
                expression_free(value, bound, sink);
                let _ = bound.insert(name.clone());
            }
            Statement::Destructure {
                subject,
                tree,
                on_failure,
                ..
            } => {
                expression_free(subject, bound, sink);
                if let Some(failure) = on_failure
                    && let Some(message) = &failure.message
                {
                    expression_free(message, bound, sink);
                }
                // Assignment bindings persist for the following statements.
                decision_free(tree, bound, sink, true);
            }
            Statement::Expression { expression, .. } => expression_free(expression, bound, sink),
        }
    }
}

fn expression_free(
    expression: &Expression,
    bound: &mut HashSet<String>,
    sink: &mut dyn FnMut(&str),
) {
    match expression {
        Expression::Variable(name) => {
            if !bound.contains(name) {
                sink(name);
            }
        }
        Expression::Int(_)
        | Expression::BigInt(_)
        | Expression::Float(_)
        | Expression::String(_)
        | Expression::Nil
        | Expression::Bool(_)
        | Expression::EmptyList
        | Expression::FunctionReference { .. } => {}
        Expression::Block(statements) => {
            let mut scope = bound.clone();
            statements_free(statements, &mut scope, sink);
        }
        Expression::Call { arguments, .. } | Expression::Constructor { arguments, .. } => {
            for argument in arguments {
                expression_free(argument, bound, sink);
            }
        }
        Expression::CallValue { callee, arguments } => {
            expression_free(callee, bound, sink);
            for argument in arguments {
                expression_free(argument, bound, sink);
            }
        }
        Expression::List { elements, tail } => {
            for element in elements {
                expression_free(element, bound, sink);
            }
            if let Some(tail) = tail {
                expression_free(tail, bound, sink);
            }
        }
        Expression::IntBinary { left, right, .. }
        | Expression::IntCompare { left, right, .. }
        | Expression::FloatBinary { left, right, .. }
        | Expression::FloatCompare { left, right, .. }
        | Expression::Equality { left, right, .. }
        | Expression::BoolBinary { left, right, .. } => {
            expression_free(left, bound, sink);
            expression_free(right, bound, sink);
        }
        Expression::StringConcat(left, right) => {
            expression_free(left, bound, sink);
            expression_free(right, bound, sink);
        }
        Expression::BoolNot(inner) => expression_free(inner, bound, sink),
        Expression::FieldAccess { record, .. } => expression_free(record, bound, sink),
        Expression::Case { subjects, tree, .. } => {
            for subject in subjects {
                expression_free(subject, bound, sink);
            }
            decision_free(tree, bound, sink, false);
        }
        Expression::Lambda { parameters, body } => {
            let mut scope = bound.clone();
            for parameter in parameters {
                let _ = scope.insert(parameter.clone());
            }
            statements_free(body, &mut scope, sink);
        }
        Expression::BitArray(segments) => {
            for segment in segments {
                expression_free(&segment.value, bound, sink);
                match &segment.kind {
                    BitSegmentKind::Int { bits, .. } | BitSegmentKind::Float { bits, .. } => {
                        expression_free(bits, bound, sink)
                    }
                    BitSegmentKind::BitArraySplice { bits: Some(bits) } => {
                        expression_free(bits, bound, sink)
                    }
                    BitSegmentKind::BitArraySplice { bits: None }
                    | BitSegmentKind::String { .. }
                    | BitSegmentKind::Codepoint { .. } => {}
                }
            }
        }
        Expression::Echo { value, message, .. } => {
            expression_free(value, bound, sink);
            if let Some(message) = message {
                expression_free(message, bound, sink);
            }
        }
        Expression::Panic { message, .. } => {
            if let Some(message) = message {
                expression_free(message, bound, sink);
            }
        }
    }
}

fn decision_free(
    decision: &Decision,
    bound: &mut HashSet<String>,
    sink: &mut dyn FnMut(&str),
    bindings_persist: bool,
) {
    match decision {
        Decision::Run { bindings, body } => {
            let mut scope = if bindings_persist {
                None
            } else {
                Some(bound.clone())
            };
            let scope = scope.as_mut().unwrap_or(bound);
            for (name, value) in bindings {
                bound_free(value, scope, sink);
                let _ = scope.insert(name.clone());
            }
            statements_free(body, scope, sink);
        }
        Decision::Switch {
            choices, fallback, ..
        } => {
            for (check, decision) in choices {
                check_free(check, bound, sink);
                decision_free(decision, bound, sink, bindings_persist);
            }
            decision_free(fallback, bound, sink, bindings_persist);
        }
        Decision::Guard {
            bindings,
            guard,
            if_true,
            if_false,
        } => {
            let mut scope = bound.clone();
            for (name, value) in bindings {
                bound_free(value, &mut scope, sink);
                let _ = scope.insert(name.clone());
            }
            expression_free(guard, &mut scope, sink);
            statements_free(if_true, &mut scope, sink);
            decision_free(if_false, bound, sink, bindings_persist);
        }
        Decision::Fail => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_variable_analysis() {
        // fn(x) { let y = x + outer case z { n -> n + y } }
        let body = vec![
            Statement::Let {
                name: "y".into(),
                value: Expression::IntBinary {
                    operator: IntOperator::Add,
                    left: Box::new(Expression::Variable("x".into())),
                    right: Box::new(Expression::Variable("outer".into())),
                },
                line: 0,
            },
            Statement::expression(Expression::Case {
                subjects: vec![Expression::Variable("z".into())],
                subject_ids: vec![0],
                tree: Decision::Switch {
                    var: 0,
                    choices: vec![],
                    fallback: Box::new(Decision::Run {
                        bindings: vec![("n".into(), Bound::Variable(0))],
                        body: vec![Statement::expression(Expression::IntBinary {
                            operator: IntOperator::Add,
                            left: Box::new(Expression::Variable("n".into())),
                            right: Box::new(Expression::Variable("y".into())),
                        })],
                    }),
                    fallback_fields: vec![],
                },
            }),
        ];
        let bound = HashSet::from(["x".to_string()]);
        let free: Vec<String> = free_variables(&body, &bound).into_iter().collect();
        // `x` is a parameter, `y` and `n` are locally bound; `outer` and
        // `z` are captured.
        assert_eq!(free, vec!["outer".to_string(), "z".to_string()]);
    }

    #[test]
    fn nested_lambdas_compose_scopes() {
        // fn(a) { fn(b) { a + b + c } }
        let body = vec![Statement::expression(Expression::Lambda {
            parameters: vec!["b".into()],
            body: vec![Statement::expression(Expression::IntBinary {
                operator: IntOperator::Add,
                left: Box::new(Expression::IntBinary {
                    operator: IntOperator::Add,
                    left: Box::new(Expression::Variable("a".into())),
                    right: Box::new(Expression::Variable("b".into())),
                }),
                right: Box::new(Expression::Variable("c".into())),
            })],
        })];
        let bound = HashSet::from(["a".to_string()]);
        let free: Vec<String> = free_variables(&body, &bound).into_iter().collect();
        assert_eq!(free, vec!["c".to_string()]);
    }
}

fn check_free(check: &Check, bound: &mut HashSet<String>, sink: &mut dyn FnMut(&str)) {
    let Check::BitArray { reads, test } = check else {
        return;
    };
    for read in reads {
        expression_free(&read.offset, bound, sink);
        expression_free(&read.bits, bound, sink);
        // The read's name is available to everything after this check.
        let _ = bound.insert(read.name.clone());
    }
    match test {
        BitsTest::Size { bits, .. } => expression_free(bits, bound, sink),
        BitsTest::NonNegative { value } => expression_free(value, bound, sink),
        BitsTest::Bytes { offset, .. } | BitsTest::RestIsBytes { offset } => {
            expression_free(offset, bound, sink)
        }
        BitsTest::IsFiniteFloat { offset, bits, .. }
        | BitsTest::FloatEquals { offset, bits, .. } => {
            expression_free(offset, bound, sink);
            expression_free(bits, bound, sink);
        }
        BitsTest::IntEquals {
            offset,
            bits,
            value,
            ..
        } => {
            expression_free(offset, bound, sink);
            expression_free(bits, bound, sink);
            expression_free(value, bound, sink);
        }
        BitsTest::AlwaysTrue => {}
    }
}

fn bound_free(value: &Bound, bound: &mut HashSet<String>, sink: &mut dyn FnMut(&str)) {
    match value {
        Bound::Value(expression) => expression_free(expression, bound, sink),
        Bound::Variable(_) | Bound::StringSlice { .. } => {}
        Bound::BitsReadInt { offset, bits, .. } | Bound::BitsReadFloat { offset, bits, .. } => {
            expression_free(offset, bound, sink);
            expression_free(bits, bound, sink);
        }
        Bound::BitsSlice { offset, bits, .. } => {
            expression_free(offset, bound, sink);
            if let Some(bits) = bits {
                expression_free(bits, bound, sink);
            }
        }
    }
}

// -- Scalar replacement ------------------------------------------------

/// Replaces `let`-bound constructors whose every later mention is a field
/// access with one binding per field, so no record is built at all — the
/// optimization V8 calls scalar replacement, applied here at the IR level
/// before code generation. Conservative by construction: a candidate is
/// dropped if its name is ever re-bound anywhere in the rest of its
/// sequence (so rewriting needs no scope tracking), or if any mention is
/// anything but a field access. Field bindings are named `name@index`,
/// which no source-level identifier can collide with.
pub fn scalar_replace(module: &mut Module) {
    for function in &mut module.functions {
        if let Function::Defined { body, .. } = function {
            scalar_replace_statements(body);
        }
    }
}

fn scalar_replace_statements(statements: &mut Vec<Statement>) {
    // Inner sequences first, so a nested candidate does not hide behind
    // an outer rewrite (and vice versa the outer pass sees final shapes).
    for statement in statements.iter_mut() {
        match statement {
            Statement::Let { value, .. } => scalar_replace_expression(value),
            Statement::Expression { expression, .. } => scalar_replace_expression(expression),
            Statement::Destructure {
                subject,
                tree,
                on_failure,
                ..
            } => {
                scalar_replace_expression(subject);
                scalar_replace_decision(tree);
                if let Some(failure) = on_failure
                    && let Some(message) = &mut failure.message
                {
                    scalar_replace_expression(message);
                }
            }
        }
    }

    let mut index = 0;
    while index < statements.len() {
        let candidate = match &statements[index] {
            Statement::Let {
                name,
                value: Expression::Constructor { arguments, .. },
                ..
            } if !arguments.is_empty()
                && !region_binds(&statements[index + 1..], name)
                && uses_are_field_accesses(
                    &statements[index + 1..],
                    name,
                    arguments.len() as u32,
                ) =>
            {
                Some(name.clone())
            }
            _ => None,
        };
        let Some(name) = candidate else {
            index += 1;
            continue;
        };
        let Statement::Let {
            value: Expression::Constructor { arguments, .. },
            line,
            ..
        } = statements.remove(index)
        else {
            unreachable!("candidate shape checked above");
        };
        let arity = arguments.len() as u32;
        for (offset, argument) in arguments.into_iter().enumerate() {
            statements.insert(
                index + offset,
                Statement::Let {
                    name: format!("{name}@{offset}"),
                    value: argument,
                    line,
                },
            );
        }
        let after = index + arity as usize;
        rewrite_field_accesses(&mut statements[after..], &name, arity);
        // The freshly inserted field bindings may themselves be
        // constructors with field-access-only uses (nested tuples), so
        // the scan resumes at the first of them rather than after.
    }
}

fn scalar_replace_expression(expression: &mut Expression) {
    visit_expression(expression, &mut |inner| {
        match inner {
            Expression::Block(statements) | Expression::Lambda { body: statements, .. } => {
                scalar_replace_statements(statements);
                false
            }
            Expression::Case { subjects, tree, .. } => {
                for subject in subjects {
                    scalar_replace_expression(subject);
                }
                scalar_replace_decision(tree);
                false
            }
            _ => true,
        }
    });
}

fn scalar_replace_decision(decision: &mut Decision) {
    match decision {
        Decision::Run { body, .. } => scalar_replace_statements(body),
        Decision::Switch {
            choices, fallback, ..
        } => {
            for (_, choice) in choices {
                scalar_replace_decision(choice);
            }
            scalar_replace_decision(fallback);
        }
        Decision::Guard {
            guard,
            if_true,
            if_false,
            ..
        } => {
            scalar_replace_expression(guard);
            scalar_replace_statements(if_true);
            scalar_replace_decision(if_false);
        }
        Decision::Fail => {}
    }
}

/// Whether any construct in these statements (or anything nested in them)
/// binds `name`: a `let`, a lambda parameter, a pattern or guard binding,
/// or a materialized bit segment read.
fn region_binds(statements: &[Statement], name: &str) -> bool {
    statements.iter().any(|statement| match statement {
        Statement::Let {
            name: bound,
            value,
            ..
        } => bound == name || expression_binds(value, name),
        Statement::Expression { expression, .. } => expression_binds(expression, name),
        Statement::Destructure {
            subject,
            tree,
            on_failure,
            ..
        } => {
            expression_binds(subject, name)
                || decision_binds(tree, name)
                || on_failure.as_ref().is_some_and(|failure| {
                    failure
                        .message
                        .as_ref()
                        .is_some_and(|message| expression_binds(message, name))
                })
        }
    })
}

fn expression_binds(expression: &Expression, name: &str) -> bool {
    let mut found = false;
    // The visitor takes `&mut` for the rewriter's sake; this check only
    // reads.
    let mut expression = expression.clone();
    visit_expression(&mut expression, &mut |inner| {
        if found {
            return false;
        }
        match inner {
            Expression::Block(statements) => {
                found |= region_binds(statements, name);
                false
            }
            Expression::Lambda { parameters, body } => {
                found |= parameters.iter().any(|parameter| parameter == name)
                    || region_binds(body, name);
                false
            }
            Expression::Case { subjects, tree, .. } => {
                found |= subjects
                    .iter()
                    .any(|subject| expression_binds(subject, name))
                    || decision_binds(tree, name);
                false
            }
            _ => true,
        }
    });
    found
}

fn decision_binds(decision: &Decision, name: &str) -> bool {
    let bindings_bind = |bindings: &[(String, Bound)]| {
        bindings.iter().any(|(bound_name, bound)| {
            bound_name == name
                || match bound {
                    Bound::Value(expression) => expression_binds(expression, name),
                    Bound::Variable(_) | Bound::StringSlice { .. } => false,
                    Bound::BitsReadInt { offset, bits, .. }
                    | Bound::BitsReadFloat { offset, bits, .. } => {
                        expression_binds(offset, name) || expression_binds(bits, name)
                    }
                    Bound::BitsSlice { offset, bits, .. } => {
                        expression_binds(offset, name)
                            || bits.as_ref().is_some_and(|bits| expression_binds(bits, name))
                    }
                }
        })
    };
    match decision {
        Decision::Run { bindings, body } => bindings_bind(bindings) || region_binds(body, name),
        Decision::Switch {
            choices, fallback, ..
        } => {
            choices.iter().any(|(check, choice)| {
                check_binds(check, name) || decision_binds(choice, name)
            }) || decision_binds(fallback, name)
        }
        Decision::Guard {
            bindings,
            guard,
            if_true,
            if_false,
        } => {
            bindings_bind(bindings)
                || expression_binds(guard, name)
                || region_binds(if_true, name)
                || decision_binds(if_false, name)
        }
        Decision::Fail => false,
    }
}

fn check_binds(check: &Check, name: &str) -> bool {
    match check {
        Check::BitArray { reads, .. } => reads.iter().any(|read| read.name == name),
        Check::Int(_)
        | Check::BigInt(_)
        | Check::Float(_)
        | Check::String(_)
        | Check::Bool(_)
        | Check::Nil
        | Check::EmptyList
        | Check::Variant { .. }
        | Check::Always { .. }
        | Check::NonEmptyList { .. }
        | Check::StringPrefix { .. } => false,
    }
}

/// Whether every mention of `name` in these statements is a field access
/// with an index inside the arity. Assumes `name` is not re-bound in the
/// region ([`region_binds`] is checked first).
fn uses_are_field_accesses(statements: &[Statement], name: &str, arity: u32) -> bool {
    let mut ok = true;
    // Cloning to reuse the mutable walker; this check only reads.
    let mut statements = statements.to_vec();
    visit_statements(&mut statements, &mut |expression| {
        if !ok {
            return false;
        }
        match expression {
            Expression::FieldAccess { record, index } => {
                if let Expression::Variable(mentioned) = record.as_ref()
                    && mentioned == name
                {
                    ok &= *index < arity;
                    return false;
                }
                true
            }
            Expression::Variable(mentioned) => {
                if mentioned == name {
                    ok = false;
                }
                false
            }
            _ => true,
        }
    });
    ok
}

/// Replaces every `name.index` with the variable `name@index`. Assumes
/// [`uses_are_field_accesses`] held, so no other mention exists.
fn rewrite_field_accesses(statements: &mut [Statement], name: &str, arity: u32) {
    visit_statements(statements, &mut |expression| {
        if let Expression::FieldAccess { record, index } = expression
            && let Expression::Variable(mentioned) = record.as_ref()
            && mentioned == name
        {
            debug_assert!(*index < arity);
            *expression = Expression::Variable(format!("{name}@{index}"));
            return false;
        }
        true
    });
}

/// Calls `visit` on every expression in the statements, recursively —
/// decision trees, bindings, bit segments, and failure messages included.
/// The visitor returns whether to descend into the expression's children.
fn visit_statements(statements: &mut [Statement], visit: &mut dyn FnMut(&mut Expression) -> bool) {
    for statement in statements {
        match statement {
            Statement::Let { value, .. } => visit_expression(value, visit),
            Statement::Expression { expression, .. } => visit_expression(expression, visit),
            Statement::Destructure {
                subject,
                tree,
                on_failure,
                ..
            } => {
                visit_expression(subject, visit);
                visit_decision(tree, visit);
                if let Some(failure) = on_failure
                    && let Some(message) = &mut failure.message
                {
                    visit_expression(message, visit);
                }
            }
        }
    }
}

fn visit_expression(expression: &mut Expression, visit: &mut dyn FnMut(&mut Expression) -> bool) {
    if !visit(expression) {
        return;
    }
    match expression {
        Expression::Int(_)
        | Expression::BigInt(_)
        | Expression::Float(_)
        | Expression::String(_)
        | Expression::Nil
        | Expression::Bool(_)
        | Expression::Variable(_)
        | Expression::EmptyList
        | Expression::FunctionReference { .. } => {}
        Expression::Block(statements) | Expression::Lambda { body: statements, .. } => {
            visit_statements(statements, visit)
        }
        Expression::Call { arguments, .. } | Expression::Constructor { arguments, .. } => {
            for argument in arguments {
                visit_expression(argument, visit);
            }
        }
        Expression::CallValue { callee, arguments } => {
            visit_expression(callee, visit);
            for argument in arguments {
                visit_expression(argument, visit);
            }
        }
        Expression::IntBinary { left, right, .. }
        | Expression::IntCompare { left, right, .. }
        | Expression::FloatBinary { left, right, .. }
        | Expression::FloatCompare { left, right, .. }
        | Expression::Equality { left, right, .. }
        | Expression::StringConcat(left, right) => {
            visit_expression(left, visit);
            visit_expression(right, visit);
        }
        Expression::BoolBinary { left, right, .. } => {
            visit_expression(left, visit);
            visit_expression(right, visit);
        }
        Expression::BoolNot(inner) => visit_expression(inner, visit),
        Expression::FieldAccess { record, .. } => visit_expression(record, visit),
        Expression::Case { subjects, tree, .. } => {
            for subject in subjects {
                visit_expression(subject, visit);
            }
            visit_decision(tree, visit);
        }
        Expression::List { elements, tail } => {
            for element in elements {
                visit_expression(element, visit);
            }
            if let Some(tail) = tail {
                visit_expression(tail, visit);
            }
        }
        Expression::BitArray(segments) => {
            for segment in segments {
                visit_expression(&mut segment.value, visit);
                match &mut segment.kind {
                    BitSegmentKind::Int { bits, .. } | BitSegmentKind::Float { bits, .. } => {
                        visit_expression(bits, visit)
                    }
                    BitSegmentKind::BitArraySplice { bits } => {
                        if let Some(bits) = bits {
                            visit_expression(bits, visit);
                        }
                    }
                    BitSegmentKind::String { .. } | BitSegmentKind::Codepoint { .. } => {}
                }
            }
        }
        Expression::Echo { value, message, .. } => {
            visit_expression(value, visit);
            if let Some(message) = message {
                visit_expression(message, visit);
            }
        }
        Expression::Panic { message, .. } => {
            if let Some(message) = message {
                visit_expression(message, visit);
            }
        }
    }
}

fn visit_decision(decision: &mut Decision, visit: &mut dyn FnMut(&mut Expression) -> bool) {
    let visit_bindings = |bindings: &mut Vec<(String, Bound)>,
                          visit: &mut dyn FnMut(&mut Expression) -> bool| {
        for (_, bound) in bindings {
            match bound {
                Bound::Value(expression) => visit_expression(expression, visit),
                Bound::Variable(_) | Bound::StringSlice { .. } => {}
                Bound::BitsReadInt { offset, bits, .. }
                | Bound::BitsReadFloat { offset, bits, .. } => {
                    visit_expression(offset, visit);
                    visit_expression(bits, visit);
                }
                Bound::BitsSlice { offset, bits, .. } => {
                    visit_expression(offset, visit);
                    if let Some(bits) = bits {
                        visit_expression(bits, visit);
                    }
                }
            }
        }
    };
    match decision {
        Decision::Run { bindings, body } => {
            visit_bindings(bindings, visit);
            visit_statements(body, visit);
        }
        Decision::Switch {
            choices, fallback, ..
        } => {
            for (check, choice) in choices {
                visit_check(check, visit);
                visit_decision(choice, visit);
            }
            visit_decision(fallback, visit);
        }
        Decision::Guard {
            bindings,
            guard,
            if_true,
            if_false,
        } => {
            visit_bindings(bindings, visit);
            visit_expression(guard, visit);
            visit_statements(if_true, visit);
            visit_decision(if_false, visit);
        }
        Decision::Fail => {}
    }
}

fn visit_check(check: &mut Check, visit: &mut dyn FnMut(&mut Expression) -> bool) {
    match check {
        Check::BitArray { reads, test } => {
            for read in reads {
                visit_expression(&mut read.offset, visit);
                visit_expression(&mut read.bits, visit);
            }
            visit_bits_test(test, visit);
        }
        Check::Int(_)
        | Check::BigInt(_)
        | Check::Float(_)
        | Check::String(_)
        | Check::Bool(_)
        | Check::Nil
        | Check::EmptyList
        | Check::Variant { .. }
        | Check::Always { .. }
        | Check::NonEmptyList { .. }
        | Check::StringPrefix { .. } => {}
    }
}

fn visit_bits_test(test: &mut BitsTest, visit: &mut dyn FnMut(&mut Expression) -> bool) {
    match test {
        BitsTest::Size { bits, .. } => visit_expression(bits, visit),
        BitsTest::NonNegative { value } => visit_expression(value, visit),
        BitsTest::Bytes { offset, .. } | BitsTest::RestIsBytes { offset } => {
            visit_expression(offset, visit)
        }
        BitsTest::IsFiniteFloat { offset, bits, .. }
        | BitsTest::FloatEquals { offset, bits, .. } => {
            visit_expression(offset, visit);
            visit_expression(bits, visit);
        }
        BitsTest::IntEquals {
            offset,
            bits,
            value,
            ..
        } => {
            visit_expression(offset, visit);
            visit_expression(bits, visit);
            visit_expression(value, visit);
        }
        BitsTest::AlwaysTrue => {}
    }
}

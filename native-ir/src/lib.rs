// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The serializable intermediate representation for the Gleam native target.
//!
//! `compiler-core` lowers typed Gleam modules into this IR at build time and
//! writes one artifact file per module; `native-generation` loads the
//! artifacts and translates them to Cranelift IR. This crate must stay free
//! of Cranelift so that `compiler-core` remains compilable to WebAssembly.

use std::collections::BTreeSet;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Bumped whenever the types in this crate change shape, so that stale
/// artifacts from previous compiler builds are rejected rather than
/// misinterpreted. bitcode is not a self-describing format.
pub const FORMAT_VERSION: u32 = 29;

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
    },
    Expression(Expression),
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
    /// `True` or `False`, represented as the tagged small integers 1 and 0.
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
    /// The empty list, a tagged immediate. Cons cells are two-field records
    /// with tag 1, built with [`Expression::Constructor`].
    EmptyList,
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
    /// standard error and evaluates to the value.
    Echo {
        kind: EchoKind,
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
    /// The subject is exactly this tagged word (`Bool`/`Nil` variants).
    Immediate(i64),
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
    /// Values that are always tagged immediates (`Bool`, `Nil`) compare as
    /// plain words.
    Immediate,
    /// Any other type: structural deep equality in the runtime, walking
    /// heap object headers.
    Deep,
}

/// How `echo` renders its value: exactly, when the compiler knew the static
/// type at the echo site, or structurally otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EchoKind {
    Structural,
    Int,
    Float,
    String,
    Bool,
    Nil,
    /// The static type is a list: the empty list is a bare tagged integer,
    /// so without this kind a top-level `echo []` would print `0`.
    List,
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
        return Err(
            "outdated or corrupt native artifact. \
Delete the project's `build` directory and rebuild."
                .into(),
        );
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
    statements_free(statements, &mut bound, &mut free);
    free
}

fn statements_free(
    statements: &[Statement],
    bound: &mut HashSet<String>,
    free: &mut BTreeSet<String>,
) {
    for statement in statements {
        match statement {
            Statement::Let { name, value } => {
                expression_free(value, bound, free);
                let _ = bound.insert(name.clone());
            }
            Statement::Destructure { subject, tree, on_failure, .. } => {
                expression_free(subject, bound, free);
                if let Some(failure) = on_failure
                    && let Some(message) = &failure.message
                {
                    expression_free(message, bound, free);
                }
                // Assignment bindings persist for the following statements.
                decision_free(tree, bound, free, true);
            }
            Statement::Expression(expression) => expression_free(expression, bound, free),
        }
    }
}

fn expression_free(
    expression: &Expression,
    bound: &mut HashSet<String>,
    free: &mut BTreeSet<String>,
) {
    match expression {
        Expression::Variable(name) => {
            if !bound.contains(name) {
                let _ = free.insert(name.clone());
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
            statements_free(statements, &mut scope, free);
        }
        Expression::Call { arguments, .. } | Expression::Constructor { arguments, .. } => {
            for argument in arguments {
                expression_free(argument, bound, free);
            }
        }
        Expression::CallValue { callee, arguments } => {
            expression_free(callee, bound, free);
            for argument in arguments {
                expression_free(argument, bound, free);
            }
        }
        Expression::IntBinary { left, right, .. }
        | Expression::IntCompare { left, right, .. }
        | Expression::FloatBinary { left, right, .. }
        | Expression::FloatCompare { left, right, .. }
        | Expression::Equality { left, right, .. }
        | Expression::BoolBinary { left, right, .. } => {
            expression_free(left, bound, free);
            expression_free(right, bound, free);
        }
        Expression::StringConcat(left, right) => {
            expression_free(left, bound, free);
            expression_free(right, bound, free);
        }
        Expression::BoolNot(inner) => expression_free(inner, bound, free),
        Expression::FieldAccess { record, .. } => expression_free(record, bound, free),
        Expression::Case { subjects, tree, .. } => {
            for subject in subjects {
                expression_free(subject, bound, free);
            }
            decision_free(tree, bound, free, false);
        }
        Expression::Lambda { parameters, body } => {
            let mut scope = bound.clone();
            for parameter in parameters {
                let _ = scope.insert(parameter.clone());
            }
            statements_free(body, &mut scope, free);
        }
        Expression::BitArray(segments) => {
            for segment in segments {
                expression_free(&segment.value, bound, free);
                match &segment.kind {
                    BitSegmentKind::Int { bits, .. } | BitSegmentKind::Float { bits, .. } => {
                        expression_free(bits, bound, free)
                    }
                    BitSegmentKind::BitArraySplice { bits: Some(bits) } => {
                        expression_free(bits, bound, free)
                    }
                    BitSegmentKind::BitArraySplice { bits: None }
                    | BitSegmentKind::String { .. }
                    | BitSegmentKind::Codepoint { .. } => {}
                }
            }
        }
        Expression::Echo { value, message, .. } => {
            expression_free(value, bound, free);
            if let Some(message) = message {
                expression_free(message, bound, free);
            }
        }
        Expression::Panic { message, .. } => {
            if let Some(message) = message {
                expression_free(message, bound, free);
            }
        }
    }
}

fn decision_free(
    decision: &Decision,
    bound: &mut HashSet<String>,
    free: &mut BTreeSet<String>,
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
                bound_free(value, scope, free);
                let _ = scope.insert(name.clone());
            }
            statements_free(body, scope, free);
        }
        Decision::Switch {
            choices, fallback, ..
        } => {
            for (check, decision) in choices {
                check_free(check, bound, free);
                decision_free(decision, bound, free, bindings_persist);
            }
            decision_free(fallback, bound, free, bindings_persist);
        }
        Decision::Guard {
            bindings,
            guard,
            if_true,
            if_false,
        } => {
            let mut scope = bound.clone();
            for (name, value) in bindings {
                bound_free(value, &mut scope, free);
                let _ = scope.insert(name.clone());
            }
            expression_free(guard, &mut scope, free);
            statements_free(if_true, &mut scope, free);
            decision_free(if_false, bound, free, bindings_persist);
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
            },
            Statement::Expression(Expression::Case {
                subjects: vec![Expression::Variable("z".into())],
                subject_ids: vec![0],
                tree: Decision::Switch {
                    var: 0,
                    choices: vec![],
                    fallback: Box::new(Decision::Run {
                        bindings: vec![("n".into(), Bound::Variable(0))],
                        body: vec![Statement::Expression(Expression::IntBinary {
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
        let body = vec![Statement::Expression(Expression::Lambda {
            parameters: vec!["b".into()],
            body: vec![Statement::Expression(Expression::IntBinary {
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

fn check_free(check: &Check, bound: &mut HashSet<String>, free: &mut BTreeSet<String>) {
    let Check::BitArray { reads, test } = check else {
        return;
    };
    for read in reads {
        expression_free(&read.offset, bound, free);
        expression_free(&read.bits, bound, free);
        // The read's name is available to everything after this check.
        let _ = bound.insert(read.name.clone());
    }
    match test {
        BitsTest::Size { bits, .. } => expression_free(bits, bound, free),
        BitsTest::NonNegative { value } => expression_free(value, bound, free),
        BitsTest::Bytes { offset, .. } | BitsTest::RestIsBytes { offset } => {
            expression_free(offset, bound, free)
        }
        BitsTest::IsFiniteFloat { offset, bits, .. } => {
            expression_free(offset, bound, free);
            expression_free(bits, bound, free);
        }
        BitsTest::AlwaysTrue => {}
    }
}

fn bound_free(value: &Bound, bound: &mut HashSet<String>, free: &mut BTreeSet<String>) {
    match value {
        Bound::Value(expression) => expression_free(expression, bound, free),
        Bound::Variable(_) | Bound::StringSlice { .. } => {}
        Bound::BitsReadInt { offset, bits, .. } | Bound::BitsReadFloat { offset, bits, .. } => {
            expression_free(offset, bound, free);
            expression_free(bits, bound, free);
        }
        Bound::BitsSlice { offset, bits, .. } => {
            expression_free(offset, bound, free);
            if let Some(bits) = bits {
                expression_free(bits, bound, free);
            }
        }
    }
}

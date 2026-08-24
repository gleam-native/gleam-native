// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The serializable intermediate representation for the Gleam native target.
//!
//! `compiler-core` lowers typed Gleam modules into this IR at build time and
//! writes one artifact file per module; `native-generation` loads the
//! artifacts and translates them to Cranelift IR. This crate must stay free
//! of Cranelift so that `compiler-core` remains compilable to WebAssembly.

use serde::{Deserialize, Serialize};

/// Bumped whenever the types in this crate change shape, so that stale
/// artifacts from previous compiler builds are rejected rather than
/// misinterpreted. bitcode is not a self-describing format.
pub const FORMAT_VERSION: u32 = 17;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    pub version: u32,
    pub module: Module,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// The Gleam module name, with `/` separators, e.g. `gleam/wibble`.
    pub name: String,
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
    Let { name: String, value: Expression },
    Expression(Expression),
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
    /// word followed by the field values.
    Constructor {
        tag: u32,
        arguments: Vec<Expression>,
    },
    /// Reading field `index` out of a custom type record.
    FieldAccess {
        record: Box<Expression>,
        index: u32,
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

pub fn encode(module: &Module) -> Result<Vec<u8>, bitcode::Error> {
    bitcode::serialize(&Artifact {
        version: FORMAT_VERSION,
        module: module.clone(),
    })
}

pub fn decode(bytes: &[u8]) -> Result<Module, String> {
    let artifact: Artifact = bitcode::deserialize(bytes).map_err(|error| {
        format!(
            "corrupt or outdated native artifact ({error}). \
Delete the project's `build` directory and rebuild."
        )
    })?;
    if artifact.version != FORMAT_VERSION {
        return Err(format!(
            "native artifact format version {} does not match compiler version {}. \
Delete the project's `build` directory and rebuild.",
            artifact.version, FORMAT_VERSION
        ));
    }
    Ok(artifact.module)
}

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
pub const FORMAT_VERSION: u32 = 10;

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
    StringConcat(Box<Expression>, Box<Expression>),
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

// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! A seeded, type-directed generator of total, panic-free Gleam programs
//! using only language features (no standard library), for differential
//! testing of the native target against Erlang.
//!
//! Programs are well-typed by construction: every expression is generated
//! for a target type, every `case` ends with a catch-all clause, recursion
//! is fuel-bounded, and the features that legitimately diverge between
//! targets are avoided — `echo` of bare bit arrays and functions (their
//! rendering differs), float multiplication and division (Erlang raises
//! on overflow to infinity where native saturates), and runtime failures
//! (panic report formats are target-specific).

/// SplitMix64: tiny, deterministic, seedable.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Mixes a base seed and an iteration index into an independent
    /// program seed.
    pub fn mix(base: u64, index: u64) -> u64 {
        let mut rng = Rng::new(base ^ index.wrapping_mul(0x2545_F491_4F6C_DD1D));
        rng.next()
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound.max(1)
    }

    fn range(&mut self, low: i64, high: i64) -> i64 {
        low + self.below((high - low + 1) as u64) as i64
    }

    fn chance(&mut self, numerator: u64, denominator: u64) -> bool {
        self.below(denominator) < numerator
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Type {
    Int,
    Float,
    Bool,
    Nil,
    String,
    List(Box<Type>),
    Tuple(Vec<Type>),
    Custom(usize),
    Fn(Vec<Type>, Box<Type>),
}

struct Variant {
    name: String,
    fields: Vec<(String, Type)>,
}

struct CustomType {
    name: String,
    variants: Vec<Variant>,
}

struct FnSig {
    name: String,
    params: Vec<Type>,
    ret: Type,
    /// A fuel-bounded recursive function: its first argument must be a
    /// small literal, or generated call chains could run forever.
    fueled: bool,
}

struct Generator {
    rng: Rng,
    types: Vec<CustomType>,
    functions: Vec<FnSig>,
    /// How many of `functions` may be called from the code being
    /// generated right now (only earlier-defined ones, so call graphs stay
    /// acyclic apart from the fuelled self-recursion).
    callable: usize,
    /// In-scope let/pattern bindings.
    scope: Vec<(String, Type)>,
    counter: u32,
}

pub fn program(seed: u64) -> String {
    let mut generator = Generator {
        rng: Rng::new(seed),
        types: Vec::new(),
        functions: Vec::new(),
        callable: 0,
        scope: Vec::new(),
        counter: 0,
    };
    generator.build()
}

impl Generator {
    fn fresh(&mut self, prefix: &str) -> String {
        self.counter += 1;
        format!("{prefix}{}", self.counter)
    }

    fn build(&mut self) -> String {
        let mut out = String::new();
        self.define_custom_types(&mut out);
        self.define_functions(&mut out);
        self.define_main(&mut out);
        out
    }

    // -- Types --------------------------------------------------------

    /// Types a generated value can have; `echo`-safe ones only (bit
    /// arrays and functions render differently across targets, so they
    /// are produced only in positions that never reach `echo`).
    fn value_type(&mut self, depth: u32) -> Type {
        match self.rng.below(if depth == 0 { 6 } else { 9 }) {
            0 => Type::Int,
            1 => Type::Float,
            2 => Type::Bool,
            3 => Type::String,
            4 => Type::Nil,
            5 => Type::Custom(self.rng.below(self.types.len() as u64) as usize),
            6 => Type::List(Box::new(self.value_type(depth.saturating_sub(1)))),
            _ => {
                let len = self.rng.range(2, 3);
                let elements = (0..len)
                    .map(|_| self.value_type(depth.saturating_sub(1)))
                    .collect();
                Type::Tuple(elements)
            }
        }
    }

    fn type_name(&self, type_: &Type) -> String {
        match type_ {
            Type::Int => "Int".into(),
            Type::Float => "Float".into(),
            Type::Bool => "Bool".into(),
            Type::Nil => "Nil".into(),
            Type::String => "String".into(),
            Type::List(element) => format!("List({})", self.type_name(element)),
            Type::Tuple(elements) => {
                let elements: Vec<String> = elements.iter().map(|t| self.type_name(t)).collect();
                format!("#({})", elements.join(", "))
            }
            Type::Custom(index) => self.types[*index].name.clone(),
            Type::Fn(params, ret) => {
                let params: Vec<String> = params.iter().map(|t| self.type_name(t)).collect();
                format!("fn({}) -> {}", params.join(", "), self.type_name(ret))
            }
        }
    }

    fn define_custom_types(&mut self, out: &mut String) {
        // The first custom type is single-variant, so field access and
        // record updates are always valid on it.
        let field_pool = [
            Type::Int,
            Type::Float,
            Type::Bool,
            Type::String,
            Type::List(Box::new(Type::Int)),
        ];
        let mut fields = Vec::new();
        for index in 0..self.rng.range(2, 3) {
            let type_ = field_pool[self.rng.below(field_pool.len() as u64) as usize].clone();
            fields.push((format!("field{index}"), type_));
        }
        self.types.push(CustomType {
            name: "Single".into(),
            variants: vec![Variant {
                name: "Single".into(),
                fields,
            }],
        });

        // One or two multi-variant types for case matching.
        for type_index in 0..self.rng.range(1, 2) {
            let mut variants = Vec::new();
            for variant_index in 0..self.rng.range(2, 3) {
                let mut fields = Vec::new();
                for field_index in 0..self.rng.range(0, 2) {
                    let type_ =
                        field_pool[self.rng.below(field_pool.len() as u64) as usize].clone();
                    fields.push((
                        format!("value{type_index}x{variant_index}x{field_index}"),
                        type_,
                    ));
                }
                variants.push(Variant {
                    name: format!("Variant{type_index}x{variant_index}"),
                    fields,
                });
            }
            variants.push(Variant {
                name: format!("Fallback{type_index}"),
                fields: vec![],
            });
            self.types.push(CustomType {
                name: format!("Shape{type_index}"),
                variants,
            });
        }

        for custom in &self.types {
            out.push_str(&format!("pub type {} {{\n", custom.name));
            for variant in &custom.variants {
                if variant.fields.is_empty() {
                    out.push_str(&format!("  {}\n", variant.name));
                } else {
                    let fields: Vec<String> = variant
                        .fields
                        .iter()
                        .map(|(name, type_)| format!("{name}: {}", self.type_name(type_)))
                        .collect();
                    out.push_str(&format!("  {}({})\n", variant.name, fields.join(", ")));
                }
            }
            out.push_str("}\n\n");
        }
    }

    // -- Functions ----------------------------------------------------

    fn define_functions(&mut self, out: &mut String) {
        // Fuel-bounded recursive functions over an accumulator, mixing
        // tail and non-tail self calls.
        for index in 0..self.rng.range(1, 2) {
            let acc = match self.rng.below(3) {
                0 => Type::Int,
                1 => Type::String,
                _ => Type::List(Box::new(Type::Int)),
            };
            let name = format!("spin{index}");
            self.functions.push(FnSig {
                name: name.clone(),
                params: vec![Type::Int, acc.clone()],
                ret: acc.clone(),
                fueled: true,
            });
            // The step is generated without `acc` in scope and combined
            // with it linearly, so the accumulator cannot be compounded
            // into exponential growth (`acc <> acc`, `acc * acc`).
            self.scope.clear();
            self.scope.push(("fuel".into(), Type::Int));
            let next = match &acc {
                Type::Int => format!("acc + {}", self.operand(&Type::Int, 2)),
                Type::String => format!("acc <> {}", self.operand(&Type::String, 2)),
                _ => format!("[{}, ..acc]", self.expression(&Type::Int, 2)),
            };
            let recurse = format!("{name}(fuel - 1, {next})");
            let recurse = if self.rng.chance(1, 2) {
                // Non-tail: combine after the call returns.
                match &acc {
                    Type::Int => format!("1 + {recurse}"),
                    Type::String => format!("\"~\" <> {recurse}"),
                    _ => format!("[fuel, ..{recurse}]"),
                }
            } else {
                recurse
            };
            out.push_str(&format!(
                "fn {name}(fuel: Int, acc: {}) -> {} {{\n  case fuel < 1 {{\n    True -> acc\n    False -> {recurse}\n  }}\n}}\n\n",
                self.type_name(&acc),
                self.type_name(&acc),
            ));
            self.callable = self.functions.len();
        }

        // Plain helpers, each free to call everything defined before it.
        for index in 0..self.rng.range(2, 4) {
            let ret = self.value_type(1);
            let params: Vec<Type> = (0..self.rng.range(1, 2))
                .map(|_| self.value_type(1))
                .collect();
            let name = format!("helper{index}");
            self.scope.clear();
            for (position, type_) in params.iter().enumerate() {
                self.scope.push((format!("p{position}"), type_.clone()));
            }
            let body = self.expression(&ret, 3);
            let parameters: Vec<String> = params
                .iter()
                .enumerate()
                .map(|(position, type_)| format!("p{position}: {}", self.type_name(type_)))
                .collect();
            out.push_str(&format!(
                "fn {name}({}) -> {} {{\n  {body}\n}}\n\n",
                parameters.join(", "),
                self.type_name(&ret),
            ));
            self.functions.push(FnSig {
                name,
                params,
                ret,
                fueled: false,
            });
            self.callable = self.functions.len();
        }
    }

    /// Whether `echo` of this type prints identically on Erlang and
    /// native. A tuple whose first element is a zero-arity constructor is
    /// indistinguishable from a record on the BEAM, so Erlang renders it
    /// as a constructor call (the `echo_non_record_atom_tag` conformance
    /// case documents this) — such types are exercised everywhere except
    /// `echo` positions.
    fn echo_safe(&self, type_: &Type) -> bool {
        match type_ {
            Type::Int | Type::Float | Type::Bool | Type::Nil | Type::String => true,
            Type::List(element) => self.echo_safe(element),
            Type::Tuple(elements) => {
                if let Some(Type::Custom(index)) = elements.first()
                    && self.types[*index]
                        .variants
                        .iter()
                        .any(|variant| variant.fields.is_empty())
                {
                    return false;
                }
                elements.iter().all(|element| self.echo_safe(element))
            }
            // Custom type fields are scalars and integer lists, which
            // cannot embed an ambiguous tuple.
            Type::Custom(_) => true,
            Type::Fn(..) => false,
        }
    }

    fn define_main(&mut self, out: &mut String) {
        self.scope.clear();
        out.push_str("pub fn main() {\n");
        for _ in 0..self.rng.range(4, 7) {
            let type_ = loop {
                let candidate = self.value_type(1);
                if self.echo_safe(&candidate) {
                    break candidate;
                }
            };
            if self.rng.chance(1, 3) {
                let value = self.expression(&type_, 3);
                out.push_str(&format!("  echo {value}\n"));
            } else {
                let name = self.fresh("top");
                let value = self.expression(&type_, 3);
                out.push_str(&format!("  let {name} = {value}\n"));
                out.push_str(&format!("  echo {name}\n"));
                self.scope.push((name, type_));
            }
        }
        out.push_str("  Nil\n}\n");
    }

    // -- Expressions --------------------------------------------------

    /// An expression of the given type, wrapped for use as an operand of
    /// an enclosing operator.
    fn operand(&mut self, type_: &Type, depth: u32) -> String {
        let (code, atomic) = self.expression_with_shape(type_, depth);
        if atomic {
            code
        } else {
            format!("{{ {code} }}")
        }
    }

    fn expression(&mut self, type_: &Type, depth: u32) -> String {
        self.expression_with_shape(type_, depth).0
    }

    /// Generates an expression; the flag is whether it can be an operand
    /// without wrapping.
    fn expression_with_shape(&mut self, type_: &Type, depth: u32) -> (String, bool) {
        if depth == 0 {
            return (self.leaf(type_), true);
        }
        let scope_marker = self.scope.len();
        let result = match self.rng.below(12) {
            // A `case` of a random subject type.
            0 | 1 => (self.case_expression(type_, depth), true),
            // A block with a fresh binding.
            2 => {
                let bound_type = self.value_type(1);
                let name = self.fresh("v");
                let value = self.expression(&bound_type, depth - 1);
                self.scope.push((name.clone(), bound_type));
                let body = self.expression(type_, depth - 1);
                (format!("{{\n  let {name} = {value}\n  {body} }}"), true)
            }
            // A call to an already-defined function; a fuelled function's
            // first argument is always a small literal so the generated
            // recursion stays bounded.
            3 | 4 => match self.callable_function(type_) {
                Some(index) => {
                    let signature = &self.functions[index];
                    let name = signature.name.clone();
                    let params = signature.params.clone();
                    let fueled = signature.fueled;
                    let arguments: Vec<String> = params
                        .iter()
                        .enumerate()
                        .map(|(position, parameter)| {
                            if fueled && position == 0 {
                                format!("{}", self.rng.range(0, 12))
                            } else {
                                self.call_argument(parameter, depth - 1)
                            }
                        })
                        .collect();
                    (format!("{name}({})", arguments.join(", ")), true)
                }
                None => (self.leaf(type_), true),
            },
            // A call through a function value (lambda or function
            // reference bound to a variable).
            5 => {
                let parameter = self.value_type(0);
                let function_type = Type::Fn(vec![parameter.clone()], Box::new(type_.clone()));
                let function = self.function_value(&function_type, depth - 1);
                let name = self.fresh("g");
                let argument = self.expression(&parameter, depth - 1);
                (
                    format!("{{\n  let {name} = {function}\n  {name}({argument}) }}"),
                    true,
                )
            }
            // Structure access and updates.
            6 => match type_ {
                Type::Custom(0) => {
                    let name = self.fresh("r");
                    let source = self.expression(&Type::Custom(0), depth - 1);
                    let field_index = self
                        .rng
                        .below(self.types[0].variants[0].fields.len() as u64)
                        as usize;
                    let (field_name, field_type) =
                        self.types[0].variants[0].fields[field_index].clone();
                    let updated = self.expression(&field_type, depth - 1);
                    (
                        format!(
                            "{{\n  let {name} = {source}\n  Single(..{name}, {field_name}: {updated}) }}"
                        ),
                        true,
                    )
                }
                _ => match self.field_of_single(type_) {
                    Some(field_name) => {
                        let name = self.fresh("r");
                        let source = self.expression(&Type::Custom(0), depth - 1);
                        (
                            format!("{{\n  let {name} = {source}\n  {name}.{field_name} }}"),
                            true,
                        )
                    }
                    None => (self.leaf(type_), true),
                },
            },
            // Bit array construction and matching, extracting a value.
            7 if *type_ == Type::Int || *type_ == Type::Float => {
                (self.bit_array_extraction(type_, depth), true)
            }
            // Tuple construction and indexing.
            7 => {
                let before = self.value_type(0);
                let name = self.fresh("t");
                let tuple_type = Type::Tuple(vec![before, type_.clone()]);
                let tuple = self.expression(&tuple_type, depth - 1);
                (format!("{{\n  let {name} = {tuple}\n  {name}.1 }}"), true)
            }
            // A pipe into a single-argument function.
            8 => match self.pipe_function(type_) {
                Some(index) => {
                    let signature = &self.functions[index];
                    let name = signature.name.clone();
                    let parameter = signature.params[0].clone();
                    let argument = self.operand(&parameter, depth - 1);
                    (format!("{argument} |> {name}"), false)
                }
                None => (self.leaf(type_), true),
            },
            _ => self.typed_operation(type_, depth),
        };
        self.scope.truncate(scope_marker);
        result
    }

    /// An argument for a call: usually fresh, sometimes an in-scope
    /// variable to exercise sharing.
    fn call_argument(&mut self, type_: &Type, depth: u32) -> String {
        self.expression(type_, depth.min(2))
    }

    /// Type-specific operator productions.
    fn typed_operation(&mut self, type_: &Type, depth: u32) -> (String, bool) {
        match type_ {
            Type::Int => {
                let operator = ["+", "-", "*", "/", "%"][self.rng.below(5) as usize];
                let left = self.operand(&Type::Int, depth - 1);
                let right = self.operand(&Type::Int, depth - 1);
                (format!("{left} {operator} {right}"), false)
            }
            // Only addition and subtraction: Erlang raises on float
            // overflow to infinity where native saturates, so growth must
            // stay linear.
            Type::Float => {
                let operator = ["+.", "-."][self.rng.below(2) as usize];
                let left = self.operand(&Type::Float, depth - 1);
                let right = self.operand(&Type::Float, depth - 1);
                (format!("{left} {operator} {right}"), false)
            }
            Type::Bool => match self.rng.below(4) {
                0 => {
                    let operator = ["<", "<=", ">", ">=", "==", "!="][self.rng.below(6) as usize];
                    let left = self.operand(&Type::Int, depth - 1);
                    let right = self.operand(&Type::Int, depth - 1);
                    (format!("{left} {operator} {right}"), false)
                }
                1 => {
                    let operator = ["<.", "<=.", ">.", ">=."][self.rng.below(4) as usize];
                    let left = self.operand(&Type::Float, depth - 1);
                    let right = self.operand(&Type::Float, depth - 1);
                    (format!("{left} {operator} {right}"), false)
                }
                2 => {
                    // Structural equality at a random type.
                    let compared = self.value_type(1);
                    let operator = ["==", "!="][self.rng.below(2) as usize];
                    let left = self.operand(&compared, depth - 1);
                    let right = self.operand(&compared, depth - 1);
                    (format!("{left} {operator} {right}"), false)
                }
                _ => {
                    let operator = ["&&", "||"][self.rng.below(2) as usize];
                    let left = self.operand(&Type::Bool, depth - 1);
                    let right = self.operand(&Type::Bool, depth - 1);
                    if self.rng.chance(1, 4) {
                        (format!("!{{ {left} {operator} {right} }}"), true)
                    } else {
                        (format!("{left} {operator} {right}"), false)
                    }
                }
            },
            Type::String => {
                let left = self.operand(&Type::String, depth - 1);
                let right = self.operand(&Type::String, depth - 1);
                (format!("{left} <> {right}"), false)
            }
            Type::List(element) => {
                let element = (**element).clone();
                let head = self.expression(&element, depth - 1);
                let tail = self.expression(type_, depth - 1);
                (format!("[{head}, ..{tail}]"), true)
            }
            _ => (self.leaf(type_), true),
        }
    }

    /// A function-typed value: a lambda, or a reference to a defined
    /// function with the same signature.
    fn function_value(&mut self, type_: &Type, depth: u32) -> String {
        let Type::Fn(params, ret) = type_ else {
            unreachable!("function_value called for a non-function type");
        };
        for (index, signature) in self.functions.iter().enumerate().take(self.callable) {
            if signature.params == *params && signature.ret == **ret && self.rng.chance(1, 3) {
                return self.functions[index].name.clone();
            }
        }
        let scope_marker = self.scope.len();
        let parameters: Vec<String> = params
            .iter()
            .map(|parameter| {
                let name = self.fresh("a");
                self.scope.push((name.clone(), parameter.clone()));
                format!("{name}: {}", self.type_name(parameter))
            })
            .collect();
        let ret = (**ret).clone();
        let body = self.expression(&ret, depth.min(2));
        self.scope.truncate(scope_marker);
        format!("fn({}) {{ {body} }}", parameters.join(", "))
    }

    fn callable_function(&mut self, ret: &Type) -> Option<usize> {
        let matching: Vec<usize> = self
            .functions
            .iter()
            .take(self.callable)
            .enumerate()
            .filter(|(_, signature)| signature.ret == *ret)
            .map(|(index, _)| index)
            .collect();
        if matching.is_empty() {
            return None;
        }
        Some(matching[self.rng.below(matching.len() as u64) as usize])
    }

    fn pipe_function(&mut self, ret: &Type) -> Option<usize> {
        let matching: Vec<usize> = self
            .functions
            .iter()
            .take(self.callable)
            .enumerate()
            .filter(|(_, signature)| signature.ret == *ret && signature.params.len() == 1)
            .map(|(index, _)| index)
            .collect();
        if matching.is_empty() {
            return None;
        }
        Some(matching[self.rng.below(matching.len() as u64) as usize])
    }

    fn field_of_single(&mut self, type_: &Type) -> Option<String> {
        let fields = &self.types[0].variants[0].fields;
        let matching: Vec<&(String, Type)> = fields.iter().filter(|(_, t)| t == type_).collect();
        if matching.is_empty() {
            return None;
        }
        Some(
            matching[self.rng.below(matching.len() as u64) as usize]
                .0
                .clone(),
        )
    }

    // -- Bit arrays ---------------------------------------------------

    /// Builds a bit array and matches on it, yielding an extracted int or
    /// float (with a literal fallback, so a failed match stays total).
    fn bit_array_extraction(&mut self, type_: &Type, depth: u32) -> String {
        let name = self.fresh("b");
        let bits = self.bit_array_literal();
        let fallback = self.leaf(type_);
        let (pattern, extracted) = if *type_ == Type::Float {
            let size = ["", ":float-size(32)", ":float-little"][self.rng.below(3) as usize];
            let pattern = if size.is_empty() {
                "<<x:float, _:bits>>".to_string()
            } else {
                format!("<<x{size}, _:bits>>")
            };
            (pattern, "x".to_string())
        } else {
            match self.rng.below(5) {
                0 => ("<<x, _:bits>>".to_string(), "x".to_string()),
                1 => {
                    let size = [4, 12, 16, 24][self.rng.below(4) as usize];
                    let endian = ["", "-little"][self.rng.below(2) as usize];
                    (
                        format!("<<x:size({size}){endian}, _:bits>>"),
                        "x".to_string(),
                    )
                }
                2 => ("<<x:signed, rest:bits>>".to_string(), "x".to_string()),
                3 => {
                    // A dynamic size read from an earlier segment.
                    ("<<len, x:size(len), _:bits>>".to_string(), "x".to_string())
                }
                _ => {
                    let literal = self.rng.range(0, 255);
                    (format!("<<{literal}, x, _:bits>>"), "x".to_string())
                }
            }
        };
        let _ = depth;
        format!(
            "{{\n  let {name} = {bits}\n  case {name} {{\n    {pattern} -> {extracted}\n    _ -> {fallback}\n  }} }}"
        )
    }

    fn bit_array_literal(&mut self) -> String {
        let mut segments = Vec::new();
        for _ in 0..self.rng.range(1, 4) {
            segments.push(match self.rng.below(6) {
                0 => format!("{}", self.rng.range(0, 255)),
                1 => {
                    let size = [4, 8, 12, 16, 32][self.rng.below(5) as usize];
                    let endian = ["", "-little"][self.rng.below(2) as usize];
                    format!("{}:size({size}){endian}", self.rng.range(-500, 5000))
                }
                2 => format!(
                    "{}.{}:float",
                    self.rng.range(-100, 100),
                    self.rng.below(100)
                ),
                3 => format!(
                    "{}.{}:float-size(32)",
                    self.rng.range(-50, 50),
                    self.rng.below(10)
                ),
                4 => format!("\"{}\":utf8", self.identifier_text()),
                _ => format!("{}:size(3)", self.rng.range(0, 7)),
            });
        }
        format!("<<{}>>", segments.join(", "))
    }

    // -- Case expressions and patterns --------------------------------

    fn case_expression(&mut self, type_: &Type, depth: u32) -> String {
        let subject_type = self.value_type(1);
        let subject = self.expression(&subject_type, depth - 1);
        let mut clauses = Vec::new();
        for _ in 0..self.rng.range(1, 3) {
            let scope_marker = self.scope.len();
            // Alternative patterns are only legal at the top level of a
            // clause, and every alternative must bind the same names, so
            // generated alternatives are binding-free literals.
            let pattern = match self.literal_pattern(&subject_type) {
                Some(first) if self.rng.chance(1, 5) => {
                    let second = self
                        .literal_pattern(&subject_type)
                        .expect("a second literal pattern for the same type");
                    format!("{first} | {second}")
                }
                _ => self.pattern(&subject_type, depth),
            };
            let guard = if self.rng.chance(1, 4) {
                format!(" if {}", self.guard_expression())
            } else {
                String::new()
            };
            let body = self.expression(type_, depth - 1);
            self.scope.truncate(scope_marker);
            clauses.push(format!("    {pattern}{guard} -> {body}"));
        }
        // The guaranteed catch-all keeps every generated match total.
        let scope_marker = self.scope.len();
        let fallback_name = self.fresh("other");
        self.scope.push((fallback_name.clone(), subject_type));
        let fallback_body = self.expression(type_, depth - 1);
        self.scope.truncate(scope_marker);
        clauses.push(format!("    {fallback_name} -> {fallback_body}"));
        format!("case {subject} {{\n{}\n  }}", clauses.join("\n"))
    }

    /// A binding-free literal pattern for the type, where one exists.
    fn literal_pattern(&mut self, type_: &Type) -> Option<String> {
        Some(match type_ {
            Type::Int => format!("{}", self.rng.range(-20, 100)),
            Type::Float => format!("{}.5", self.rng.range(-10, 10)),
            Type::Bool => (if self.rng.chance(1, 2) {
                "True"
            } else {
                "False"
            })
            .to_string(),
            Type::Nil => "Nil".to_string(),
            Type::String => format!("\"{}\"", self.identifier_text()),
            Type::List(_) => "[]".to_string(),
            Type::Custom(index) => {
                let fieldless: Vec<String> = self.types[*index]
                    .variants
                    .iter()
                    .filter(|variant| variant.fields.is_empty())
                    .map(|variant| variant.name.clone())
                    .collect();
                if fieldless.is_empty() {
                    return None;
                }
                fieldless[self.rng.below(fieldless.len() as u64) as usize].clone()
            }
            Type::Tuple(..) | Type::Fn(..) => return None,
        })
    }

    /// A pattern for the subject type, pushing its bindings into scope.
    fn pattern(&mut self, type_: &Type, depth: u32) -> String {
        match type_ {
            Type::Int => match self.rng.below(3) {
                0 | 1 => format!("{}", self.rng.range(-20, 100)),
                _ => self.binding(type_),
            },
            Type::Float => {
                if self.rng.chance(1, 2) {
                    format!("{}.5", self.rng.range(-10, 10))
                } else {
                    self.binding(type_)
                }
            }
            Type::Bool => (if self.rng.chance(1, 2) {
                "True"
            } else {
                "False"
            })
            .to_string(),
            Type::Nil => "Nil".to_string(),
            Type::String => match self.rng.below(3) {
                0 => format!("\"{}\"", self.identifier_text()),
                1 => {
                    let rest = self.fresh("rest");
                    self.scope.push((rest.clone(), Type::String));
                    format!("\"{}\" <> {rest}", self.identifier_text())
                }
                _ => self.binding(type_),
            },
            Type::List(element) => {
                let element = (**element).clone();
                match self.rng.below(4) {
                    0 => "[]".to_string(),
                    1 => {
                        let head = self.pattern(&element, depth.saturating_sub(1));
                        format!("[{head}]")
                    }
                    2 => {
                        let head = self.pattern(&element, depth.saturating_sub(1));
                        let rest = self.fresh("rest");
                        self.scope.push((rest.clone(), type_.clone()));
                        format!("[{head}, ..{rest}]")
                    }
                    _ => self.binding(type_),
                }
            }
            Type::Tuple(elements) => {
                let elements = elements.clone();
                let parts: Vec<String> = elements
                    .iter()
                    .map(|element| self.pattern(element, depth.saturating_sub(1)))
                    .collect();
                format!("#({})", parts.join(", "))
            }
            Type::Custom(index) => {
                let index = *index;
                let variant_index =
                    self.rng.below(self.types[index].variants.len() as u64) as usize;
                let (variant_name, fields): (String, Vec<Type>) = {
                    let variant = &self.types[index].variants[variant_index];
                    (
                        variant.name.clone(),
                        variant.fields.iter().map(|(_, t)| t.clone()).collect(),
                    )
                };
                if fields.is_empty() {
                    variant_name
                } else {
                    let parts: Vec<String> = fields
                        .iter()
                        .map(|field| self.pattern(field, depth.saturating_sub(1)))
                        .collect();
                    format!("{variant_name}({})", parts.join(", "))
                }
            }
            Type::Fn(..) => self.binding(type_),
        }
    }

    fn binding(&mut self, type_: &Type) -> String {
        if self.rng.chance(1, 4) {
            return "_".to_string();
        }
        let name = self.fresh("m");
        self.scope.push((name.clone(), type_.clone()));
        name
    }

    /// A guard over in-scope bindings: comparisons, equality, and boolean
    /// connectives only (the guard-legal subset).
    fn guard_expression(&mut self) -> String {
        let comparison = |this: &mut Self| -> String {
            if let Some(name) = this.variable(&Type::Int) {
                let operator = ["<", "<=", ">", ">=", "==", "!="][this.rng.below(6) as usize];
                format!("{name} {operator} {}", this.rng.range(-10, 50))
            } else if let Some(name) = this.variable(&Type::Bool) {
                name
            } else {
                "True".to_string()
            }
        };
        let first = comparison(self);
        if self.rng.chance(1, 2) {
            let second = comparison(self);
            let operator = ["&&", "||"][self.rng.below(2) as usize];
            format!("{first} {operator} {second}")
        } else {
            first
        }
    }

    // -- Leaves -------------------------------------------------------

    fn variable(&mut self, type_: &Type) -> Option<String> {
        let matching: Vec<&(String, Type)> =
            self.scope.iter().filter(|(_, t)| t == type_).collect();
        if matching.is_empty() {
            return None;
        }
        Some(
            matching[self.rng.below(matching.len() as u64) as usize]
                .0
                .clone(),
        )
    }

    fn leaf(&mut self, type_: &Type) -> String {
        if self.rng.chance(1, 2)
            && let Some(name) = self.variable(type_)
        {
            return name;
        }
        self.literal(type_)
    }

    fn literal(&mut self, type_: &Type) -> String {
        match type_ {
            Type::Int => match self.rng.below(6) {
                0..=2 => format!("{}", self.rng.range(-100, 100)),
                3 => format!("{}", self.rng.range(i32::MIN as i64, i32::MAX as i64)),
                4 => format!("{}", self.rng.next() as i64),
                _ => {
                    // A big-integer literal beyond the 63-bit range.
                    let negative = if self.rng.chance(1, 2) { "-" } else { "" };
                    let digits: String = (0..self.rng.range(20, 30))
                        .map(|_| char::from(b'0' + self.rng.below(10) as u8))
                        .collect();
                    format!("{negative}1{digits}")
                }
            },
            Type::Float => {
                format!(
                    "{}.{}",
                    self.rng.range(-10_000, 10_000),
                    self.rng.below(1000)
                )
            }
            Type::Bool => (if self.rng.chance(1, 2) {
                "True"
            } else {
                "False"
            })
            .to_string(),
            Type::Nil => "Nil".to_string(),
            Type::String => format!("\"{}\"", self.string_text()),
            Type::List(element) => {
                let element = (**element).clone();
                let elements: Vec<String> = (0..self.rng.range(0, 3))
                    .map(|_| self.literal(&element))
                    .collect();
                format!("[{}]", elements.join(", "))
            }
            Type::Tuple(elements) => {
                let elements = elements.clone();
                let parts: Vec<String> = elements
                    .iter()
                    .map(|element| self.literal(element))
                    .collect();
                format!("#({})", parts.join(", "))
            }
            Type::Custom(index) => {
                let index = *index;
                let variant_index =
                    self.rng.below(self.types[index].variants.len() as u64) as usize;
                let fields: Vec<Type> = self.types[index].variants[variant_index]
                    .fields
                    .iter()
                    .map(|(_, t)| t.clone())
                    .collect();
                let variant_name = self.types[index].variants[variant_index].name.clone();
                if fields.is_empty() {
                    variant_name
                } else {
                    let arguments: Vec<String> =
                        fields.iter().map(|field| self.literal(field)).collect();
                    format!("{variant_name}({})", arguments.join(", "))
                }
            }
            Type::Fn(..) => self.function_value(&type_.clone(), 1),
        }
    }

    fn identifier_text(&mut self) -> String {
        let pool = b"abcdefghijklmnopqrstuvwxyz";
        (0..self.rng.range(1, 5))
            .map(|_| char::from(pool[self.rng.below(pool.len() as u64) as usize]))
            .collect()
    }

    fn string_text(&mut self) -> String {
        let pool = b"abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.!?-";
        let mut text: String = (0..self.rng.range(0, 12))
            .map(|_| char::from(pool[self.rng.below(pool.len() as u64) as usize]))
            .collect();
        // Escapes and non-ASCII, occasionally.
        if self.rng.chance(1, 5) {
            text.push_str(
                ["\\n", "\\t", "\\\\", "\\\"", "\\u{1F30D}", "é"][self.rng.below(6) as usize],
            );
        }
        text
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generation_is_deterministic_and_produces_a_program() {
        let program = super::program(42);
        assert_eq!(program, super::program(42));
        assert!(program.contains("pub fn main() {"));
        assert!(program.contains("echo "));
    }
}

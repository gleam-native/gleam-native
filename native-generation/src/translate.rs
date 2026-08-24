// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! Translation of the native IR into Cranelift IR.
//!
//! Every Gleam value is a single tagged 64-bit word; see `native-runtime` for
//! the representation. Gleam functions use Cranelift's `tail` calling
//! convention so guaranteed tail calls remain possible, while runtime and
//! external functions use the platform C convention. The host cannot call a
//! `tail` function directly, so an entry wrapper with the C convention is
//! generated for `main`.

use std::collections::HashMap;

use cranelift_codegen::ir::condcodes::{FloatCC, IntCC};
use cranelift_codegen::ir::{AbiParam, InstBuilder, MemFlagsData, Signature, Value, types};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, FuncId, Linkage, Module};

/// The symbols of the runtime's integer arithmetic slow paths.
pub const INT_ADD_SLOW: &str = "gleam_native_int_add_slow";
pub const INT_SUB_SLOW: &str = "gleam_native_int_sub_slow";
pub const INT_MUL_SLOW: &str = "gleam_native_int_mul_slow";

/// The symbols of the runtime's integer division and remainder, which have
/// no generated fast path: the zero-divisor rule and big integer operands
/// live entirely in the runtime.
pub const INT_DIV: &str = "gleam_native_int_div";
pub const INT_REM: &str = "gleam_native_int_rem";

/// The symbol of the runtime's three-way integer comparison, used when
/// either operand is a big integer.
pub const INT_COMPARE: &str = "gleam_native_int_compare";

/// The symbol of the runtime's big integer literal constructor.
pub const BIGINT_FROM_BYTES: &str = "gleam_native_bigint_from_bytes";

/// The symbol of the runtime's float constructor.
pub const FLOAT_FROM_BITS: &str = "gleam_native_float_from_bits";

/// The symbol of the runtime's string literal constructor.
pub const STRING_FROM_BYTES: &str = "gleam_native_string_from_bytes";

/// The symbol of the runtime's string concatenation function.
pub const STRING_CONCAT: &str = "gleam_native_string_concat";

/// The symbol of the runtime's panic/todo report-and-abort function.
pub const PANIC: &str = "gleam_native_panic";

/// The symbol of the generated C-convention wrapper around `main`.
pub const ENTRY_SYMBOL: &str = "gleam_native_main_wrapper";

const NIL: i64 = 1;

pub fn mangle(module: &str, function: &str) -> String {
    format!("gleam${}${}", module.replace('/', "$"), function)
}

fn gleam_signature(arity: usize) -> Signature {
    let mut signature = Signature::new(CallConv::Tail);
    signature
        .params
        .extend(std::iter::repeat_n(AbiParam::new(types::I64), arity));
    signature.returns.push(AbiParam::new(types::I64));
    signature
}

fn c_signature(call_conv: CallConv, arity: usize) -> Signature {
    let mut signature = Signature::new(call_conv);
    signature
        .params
        .extend(std::iter::repeat_n(AbiParam::new(types::I64), arity));
    signature.returns.push(AbiParam::new(types::I64));
    signature
}

/// The declared imports for the runtime functions generated code calls.
#[derive(Clone, Copy)]
struct RuntimeFunctions {
    int_add_slow: FuncId,
    int_sub_slow: FuncId,
    int_mul_slow: FuncId,
    int_div: FuncId,
    int_rem: FuncId,
    int_compare: FuncId,
    bigint_from_bytes: FuncId,
    float_from_bits: FuncId,
    string_from_bytes: FuncId,
    string_concat: FuncId,
    panic: FuncId,
}

impl RuntimeFunctions {
    fn declare(module: &mut impl Module) -> Result<Self, String> {
        let call_conv = module.isa().default_call_conv();
        let mut declare = |symbol: &str, arity: usize| {
            module
                .declare_function(symbol, Linkage::Import, &c_signature(call_conv, arity))
                .map_err(|error| error.to_string())
        };
        Ok(Self {
            int_add_slow: declare(INT_ADD_SLOW, 2)?,
            int_sub_slow: declare(INT_SUB_SLOW, 2)?,
            int_mul_slow: declare(INT_MUL_SLOW, 2)?,
            int_div: declare(INT_DIV, 2)?,
            int_rem: declare(INT_REM, 2)?,
            int_compare: declare(INT_COMPARE, 2)?,
            bigint_from_bytes: declare(BIGINT_FROM_BYTES, 2)?,
            float_from_bits: declare(FLOAT_FROM_BITS, 1)?,
            string_from_bytes: declare(STRING_FROM_BYTES, 2)?,
            string_concat: declare(STRING_CONCAT, 2)?,
            panic: declare(PANIC, 7)?,
        })
    }
}

pub struct Translator<'a, M: Module> {
    module: &'a mut M,
    /// Gleam (module, function) to declared Cranelift function.
    functions: HashMap<(String, String), FuncId>,
    runtime: RuntimeFunctions,
}

impl<'a, M: Module> Translator<'a, M> {
    pub fn new(module: &'a mut M) -> Result<Self, String> {
        let runtime = RuntimeFunctions::declare(module)?;
        Ok(Self {
            module,
            functions: HashMap::new(),
            runtime,
        })
    }

    pub fn function_id(&self, module: &str, function: &str) -> Option<FuncId> {
        self.functions
            .get(&(module.to_string(), function.to_string()))
            .copied()
    }

    /// First pass: declare every function of every module so that call sites
    /// can be resolved regardless of definition order.
    pub fn declare_module(&mut self, module: &native_ir::Module) -> Result<(), String> {
        let call_conv = self.module.isa().default_call_conv();
        for function in &module.functions {
            let (name, id) = match function {
                native_ir::Function::Defined {
                    name, parameters, ..
                } => {
                    let symbol = mangle(&module.name, name);
                    let id = self
                        .module
                        .declare_function(
                            &symbol,
                            Linkage::Export,
                            &gleam_signature(parameters.len()),
                        )
                        .map_err(|error| error.to_string())?;
                    (name, id)
                }
                native_ir::Function::External {
                    name,
                    arity,
                    symbol,
                } => {
                    let id = self
                        .module
                        .declare_function(
                            symbol,
                            Linkage::Import,
                            &c_signature(call_conv, *arity as usize),
                        )
                        .map_err(|error| error.to_string())?;
                    (name, id)
                }
            };
            let _ = self
                .functions
                .insert((module.name.clone(), name.clone()), id);
        }
        Ok(())
    }

    /// Second pass: translate and define the body of every Gleam function.
    pub fn define_module(&mut self, module: &native_ir::Module) -> Result<(), String> {
        for function in &module.functions {
            let native_ir::Function::Defined {
                name,
                parameters,
                body,
            } = function
            else {
                continue;
            };
            let id = self
                .function_id(&module.name, name)
                .expect("declared in first pass");
            self.define_function(id, &module.name, parameters, body)?;
        }
        Ok(())
    }

    fn define_function(
        &mut self,
        id: FuncId,
        module_name: &str,
        parameters: &[String],
        body: &[native_ir::Statement],
    ) -> Result<(), String> {
        let mut context = self.module.make_context();
        context.func.signature = gleam_signature(parameters.len());

        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let mut environment = HashMap::new();
        for (index, parameter) in parameters.iter().enumerate() {
            let value = builder.block_params(entry)[index];
            let variable = builder.declare_var(types::I64);
            builder.def_var(variable, value);
            let _ = environment.insert(parameter.clone(), variable);
        }

        let mut function_translator = FunctionTranslator {
            functions: &self.functions,
            runtime: self.runtime,
            module_name,
            module: self.module,
            builder: &mut builder,
            environment,
        };
        let result = function_translator.statements(body)?;
        builder.ins().return_(&[result]);
        builder.finalize(self.module.target_config());

        self.module
            .define_function(id, &mut context)
            .map_err(|error| error.to_string())?;
        self.module.clear_context(&mut context);
        Ok(())
    }

    /// Generates the C-convention wrapper the host uses to call `main`.
    pub fn define_entry_wrapper(&mut self, main: FuncId) -> Result<FuncId, String> {
        let call_conv = self.module.isa().default_call_conv();
        let signature = c_signature(call_conv, 0);
        let id = self
            .module
            .declare_function(ENTRY_SYMBOL, Linkage::Export, &signature)
            .map_err(|error| error.to_string())?;

        let mut context = self.module.make_context();
        context.func.signature = signature;
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let main_ref = self.module.declare_func_in_func(main, builder.func);
        let call = builder.ins().call(main_ref, &[]);
        let result = builder.inst_results(call)[0];
        builder.ins().return_(&[result]);
        builder.finalize(self.module.target_config());

        self.module
            .define_function(id, &mut context)
            .map_err(|error| error.to_string())?;
        self.module.clear_context(&mut context);
        Ok(id)
    }
}

struct FunctionTranslator<'a, 'b, M: Module> {
    functions: &'a HashMap<(String, String), FuncId>,
    runtime: RuntimeFunctions,
    module_name: &'a str,
    module: &'a mut M,
    builder: &'a mut FunctionBuilder<'b>,
    environment: HashMap<String, Variable>,
}

impl<M: Module> FunctionTranslator<'_, '_, M> {
    /// Embeds bytes as read-only constant data, yielding their address and
    /// length as values.
    fn constant_bytes(&mut self, bytes: &[u8]) -> Result<(Value, Value), String> {
        let data = self
            .module
            .declare_anonymous_data(false, false)
            .map_err(|error| error.to_string())?;
        let mut description = DataDescription::new();
        description.define(bytes.to_vec().into_boxed_slice());
        self.module
            .define_data(data, &description)
            .map_err(|error| error.to_string())?;

        let pointer_type = self.module.target_config().pointer_type();
        let data_ref = self.module.declare_data_in_func(data, self.builder.func);
        let pointer = self.builder.ins().symbol_value(pointer_type, data_ref);
        let length = self.builder.ins().iconst(types::I64, bytes.len() as i64);
        Ok((pointer, length))
    }

    /// Embeds bytes as read-only constant data and calls a two-argument
    /// runtime constructor with their address and length. Used for literals
    /// that become heap objects: big integers and strings.
    fn construct_from_constant_bytes(
        &mut self,
        bytes: &[u8],
        constructor: FuncId,
    ) -> Result<Value, String> {
        let (pointer, length) = self.constant_bytes(bytes)?;
        let constructor_ref = self.module.declare_func_in_func(constructor, self.builder.func);
        let call = self.builder.ins().call(constructor_ref, &[pointer, length]);
        Ok(self.builder.inst_results(call)[0])
    }

    /// Translates a statement sequence, returning the value of the last one.
    fn statements(&mut self, statements: &[native_ir::Statement]) -> Result<Value, String> {
        let mut last = None;
        for statement in statements {
            last = Some(match statement {
                native_ir::Statement::Expression(expression) => self.expression(expression)?,
                native_ir::Statement::Let { name, value } => {
                    let value = self.expression(value)?;
                    let variable = self.builder.declare_var(types::I64);
                    self.builder.def_var(variable, value);
                    let _ = self.environment.insert(name.clone(), variable);
                    value
                }
            });
        }
        match last {
            Some(value) => Ok(value),
            None => Ok(self.builder.ins().iconst(types::I64, NIL)),
        }
    }

    fn expression(&mut self, expression: &native_ir::Expression) -> Result<Value, String> {
        match expression {
            native_ir::Expression::Int(value) => Ok(self
                .builder
                .ins()
                .iconst(types::I64, (value << 1) | 1)),

            native_ir::Expression::BigInt(bytes) => {
                self.construct_from_constant_bytes(bytes, self.runtime.bigint_from_bytes)
            }

            native_ir::Expression::String(string) => {
                self.construct_from_constant_bytes(string.as_bytes(), self.runtime.string_from_bytes)
            }

            native_ir::Expression::Float(value) => {
                let bits = self
                    .builder
                    .ins()
                    .iconst(types::I64, value.to_bits() as i64);
                let from_bits_ref = self
                    .module
                    .declare_func_in_func(self.runtime.float_from_bits, self.builder.func);
                let call = self.builder.ins().call(from_bits_ref, &[bits]);
                Ok(self.builder.inst_results(call)[0])
            }

            native_ir::Expression::Nil => Ok(self.builder.ins().iconst(types::I64, NIL)),

            // Tagged small integers 1 and 0.
            native_ir::Expression::Bool(value) => Ok(self
                .builder
                .ins()
                .iconst(types::I64, if *value { 3 } else { 1 })),

            native_ir::Expression::Variable(name) => {
                let variable = self
                    .environment
                    .get(name)
                    .ok_or_else(|| format!("unbound variable `{name}`"))?;
                Ok(self.builder.use_var(*variable))
            }

            native_ir::Expression::Block(statements) => self.statements(statements),

            native_ir::Expression::Call {
                module,
                function,
                arguments,
            } => {
                let id = self
                    .functions
                    .get(&(module.clone(), function.clone()))
                    .copied()
                    .ok_or_else(|| format!("unknown function `{module}.{function}`"))?;
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                let function_ref = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(function_ref, &values);
                Ok(self.builder.inst_results(call)[0])
            }

            native_ir::Expression::StringConcat(left, right) => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let concat_ref = self
                    .module
                    .declare_func_in_func(self.runtime.string_concat, self.builder.func);
                let call = self.builder.ins().call(concat_ref, &[left, right]);
                Ok(self.builder.inst_results(call)[0])
            }

            native_ir::Expression::Panic {
                kind,
                message,
                function,
                line,
            } => {
                let kind = self.builder.ins().iconst(
                    types::I64,
                    match kind {
                        native_ir::PanicKind::Panic => 0,
                        native_ir::PanicKind::Todo => 1,
                    },
                );
                let message = match message {
                    Some(message) => self.expression(message)?,
                    None => self.builder.ins().iconst(types::I64, 0),
                };
                let module_name = self.module_name.to_string();
                let (module_pointer, module_length) =
                    self.constant_bytes(module_name.as_bytes())?;
                let function_name = function.clone();
                let (function_pointer, function_length) =
                    self.constant_bytes(function_name.as_bytes())?;
                let line = self.builder.ins().iconst(types::I64, *line as i64);
                let panic_ref = self.module.declare_func_in_func(self.runtime.panic, self.builder.func);
                // The runtime aborts the program and never actually returns;
                // treating this as an ordinary call keeps the block structure
                // simple, and the code after it is simply never reached.
                let call = self.builder.ins().call(
                    panic_ref,
                    &[
                        kind,
                        message,
                        module_pointer,
                        module_length,
                        function_pointer,
                        function_length,
                        line,
                    ],
                );
                Ok(self.builder.inst_results(call)[0])
            }

            native_ir::Expression::IntBinary {
                operator,
                left,
                right,
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                self.int_binary(*operator, left, right)
            }

            native_ir::Expression::IntCompare {
                operator,
                left,
                right,
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                self.int_compare(*operator, left, right)
            }

            native_ir::Expression::FloatBinary {
                operator,
                left,
                right,
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let left = self.load_float(left);
                let right = self.load_float(right);
                let result = match operator {
                    native_ir::FloatOperator::Add => self.builder.ins().fadd(left, right),
                    native_ir::FloatOperator::Subtract => self.builder.ins().fsub(left, right),
                    native_ir::FloatOperator::Multiply => self.builder.ins().fmul(left, right),
                    // Division by zero yields 0.0. Float division never
                    // traps, so the quotient is computed unconditionally and
                    // the zero-divisor case selected in.
                    native_ir::FloatOperator::Divide => {
                        let zero = self.builder.ins().f64const(0.0);
                        let divisor_is_zero =
                            self.builder.ins().fcmp(FloatCC::Equal, right, zero);
                        let quotient = self.builder.ins().fdiv(left, right);
                        self.builder.ins().select(divisor_is_zero, zero, quotient)
                    }
                };
                self.box_float(result)
            }

            native_ir::Expression::FloatCompare {
                operator,
                left,
                right,
            } => {
                let condition = match operator {
                    native_ir::CompareOperator::LessThan => FloatCC::LessThan,
                    native_ir::CompareOperator::LessThanOrEqual => FloatCC::LessThanOrEqual,
                    native_ir::CompareOperator::GreaterThan => FloatCC::GreaterThan,
                    native_ir::CompareOperator::GreaterThanOrEqual => {
                        FloatCC::GreaterThanOrEqual
                    }
                };
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let left = self.load_float(left);
                let right = self.load_float(right);
                let flag = self.builder.ins().fcmp(condition, left, right);
                Ok(self.tag_boolean_flag(flag))
            }
        }
    }

    /// Loads the f64 out of a boxed float value.
    fn load_float(&mut self, boxed: Value) -> Value {
        self.builder
            .ins()
            .load(types::F64, MemFlagsData::trusted(), boxed, 0)
    }

    /// Boxes an f64 register value via the runtime's float constructor.
    fn box_float(&mut self, value: Value) -> Result<Value, String> {
        let bits = self
            .builder
            .ins()
            .bitcast(types::I64, MemFlagsData::new(), value);
        let from_bits_ref = self
            .module
            .declare_func_in_func(self.runtime.float_from_bits, self.builder.func);
        let call = self.builder.ins().call(from_bits_ref, &[bits]);
        Ok(self.builder.inst_results(call)[0])
    }

    /// Turns an i8 comparison flag into a tagged boolean (1 or 3).
    fn tag_boolean_flag(&mut self, flag: Value) -> Value {
        let extended = self.builder.ins().uextend(types::I64, flag);
        let shifted = self.builder.ins().ishl_imm_u(extended, 1);
        self.builder.ins().bor_imm_u(shifted, 1)
    }

    /// Integer ordering comparison on tagged values. For two small integers
    /// the tagged words compare exactly like the values they encode
    /// (2x + 1 < 2y + 1 iff x < y), so the fast path is a single signed
    /// comparison. Otherwise the runtime's three-way comparison returns a
    /// tagged -1/0/1, which the same condition then compares against a
    /// tagged 0.
    fn int_compare(
        &mut self,
        operator: native_ir::CompareOperator,
        left: Value,
        right: Value,
    ) -> Result<Value, String> {
        let condition = match operator {
            native_ir::CompareOperator::LessThan => IntCC::SignedLessThan,
            native_ir::CompareOperator::LessThanOrEqual => IntCC::SignedLessThanOrEqual,
            native_ir::CompareOperator::GreaterThan => IntCC::SignedGreaterThan,
            native_ir::CompareOperator::GreaterThanOrEqual => IntCC::SignedGreaterThanOrEqual,
        };

        let fast = self.builder.create_block();
        let slow = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.append_block_param(join, types::I64);

        let both = self.builder.ins().band(left, right);
        let both_small = self.builder.ins().band_imm_u(both, 1);
        self.builder.ins().brif(both_small, fast, &[], slow, &[]);
        self.builder.seal_block(fast);
        self.builder.seal_block(slow);

        self.builder.switch_to_block(fast);
        let flag = self.builder.ins().icmp(condition, left, right);
        let result = self.tag_boolean_flag(flag);
        self.builder.ins().jump(join, &[result.into()]);

        self.builder.switch_to_block(slow);
        let compare_ref = self
            .module
            .declare_func_in_func(self.runtime.int_compare, self.builder.func);
        let call = self.builder.ins().call(compare_ref, &[left, right]);
        let ordering = self.builder.inst_results(call)[0];
        let tagged_zero = self.builder.ins().iconst(types::I64, 1);
        let flag = self.builder.ins().icmp(condition, ordering, tagged_zero);
        let result = self.tag_boolean_flag(flag);
        self.builder.ins().jump(join, &[result.into()]);
        self.builder.seal_block(join);

        self.builder.switch_to_block(join);
        Ok(self.builder.block_params(join)[0])
    }

    /// Integer arithmetic on tagged values: a fast path for two small
    /// integers whose result stays small, spilling to the runtime's big
    /// integer slow path otherwise.
    fn int_binary(
        &mut self,
        operator: native_ir::IntOperator,
        left: Value,
        right: Value,
    ) -> Result<Value, String> {
        // Division and remainder are always runtime calls: the zero-divisor
        // rule and truncation semantics live in one place there.
        let slow_function = match operator {
            native_ir::IntOperator::Add => self.runtime.int_add_slow,
            native_ir::IntOperator::Subtract => self.runtime.int_sub_slow,
            native_ir::IntOperator::Multiply => self.runtime.int_mul_slow,
            native_ir::IntOperator::Divide | native_ir::IntOperator::Remainder => {
                let function = match operator {
                    native_ir::IntOperator::Divide => self.runtime.int_div,
                    _ => self.runtime.int_rem,
                };
                let function_ref = self.module.declare_func_in_func(function, self.builder.func);
                let call = self.builder.ins().call(function_ref, &[left, right]);
                return Ok(self.builder.inst_results(call)[0]);
            }
        };

        let fast = self.builder.create_block();
        let slow = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.append_block_param(join, types::I64);

        // Take the fast path only when both operands have their small
        // integer tag bit set.
        let both = self.builder.ins().band(left, right);
        let both_small = self.builder.ins().band_imm_u(both, 1);
        self.builder.ins().brif(both_small, fast, &[], slow, &[]);
        self.builder.seal_block(fast);

        // With operands tagged as 2n + 1, each operation below computes the
        // tagged result directly, and its overflow flag coincides with the
        // mathematical result leaving the small integer range:
        //
        // - add:      (2x + 1) + (2y + 1 - 1)      == 2(x + y) + 1
        // - subtract: (2x + 1) - (2y + 1 - 1)      == 2(x - y) + 1
        // - multiply: (2x + 1 - 1) * ((2y + 1)>>1) == 2xy, then + 1
        //   (the final + 1 cannot overflow: 2xy is even, so it is at most
        //   i64::MAX - 1 when the multiplication did not overflow)
        self.builder.switch_to_block(fast);
        let (result, overflowed) = match operator {
            native_ir::IntOperator::Add => {
                let right_even = self.builder.ins().iadd_imm_s(right, -1);
                self.builder.ins().sadd_overflow(left, right_even)
            }
            native_ir::IntOperator::Subtract => {
                let right_even = self.builder.ins().iadd_imm_s(right, -1);
                self.builder.ins().ssub_overflow(left, right_even)
            }
            native_ir::IntOperator::Multiply => {
                let left_even = self.builder.ins().iadd_imm_s(left, -1);
                let right_untagged = self.builder.ins().sshr_imm_u(right, 1);
                let (product, overflowed) =
                    self.builder.ins().smul_overflow(left_even, right_untagged);
                let result = self.builder.ins().iadd_imm_s(product, 1);
                (result, overflowed)
            }
            native_ir::IntOperator::Divide | native_ir::IntOperator::Remainder => {
                unreachable!("handled by an early return above")
            }
        };
        self.builder
            .ins()
            .brif(overflowed, slow, &[], join, &[result.into()]);
        self.builder.seal_block(slow);

        self.builder.switch_to_block(slow);
        let slow_ref = self
            .module
            .declare_func_in_func(slow_function, self.builder.func);
        let call = self.builder.ins().call(slow_ref, &[left, right]);
        let slow_result = self.builder.inst_results(call)[0];
        self.builder.ins().jump(join, &[slow_result.into()]);
        self.builder.seal_block(join);

        self.builder.switch_to_block(join);
        Ok(self.builder.block_params(join)[0])
    }
}

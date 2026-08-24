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

use cranelift_codegen::ir::{AbiParam, InstBuilder, Signature, Value, types};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{FuncId, Linkage, Module};

/// The symbol of the runtime's integer addition slow path.
pub const INT_ADD_SLOW: &str = "gleam_native_int_add_slow";

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

pub struct Translator<'a, M: Module> {
    module: &'a mut M,
    /// Gleam (module, function) to declared Cranelift function.
    functions: HashMap<(String, String), FuncId>,
    int_add_slow: FuncId,
}

impl<'a, M: Module> Translator<'a, M> {
    pub fn new(module: &'a mut M) -> Result<Self, String> {
        let call_conv = module.isa().default_call_conv();
        let int_add_slow = module
            .declare_function(INT_ADD_SLOW, Linkage::Import, &c_signature(call_conv, 2))
            .map_err(|error| error.to_string())?;
        Ok(Self {
            module,
            functions: HashMap::new(),
            int_add_slow,
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
            self.define_function(id, parameters, body)?;
        }
        Ok(())
    }

    fn define_function(
        &mut self,
        id: FuncId,
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
            int_add_slow: self.int_add_slow,
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
    int_add_slow: FuncId,
    module: &'a mut M,
    builder: &'a mut FunctionBuilder<'b>,
    environment: HashMap<String, Variable>,
}

impl<M: Module> FunctionTranslator<'_, '_, M> {
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

            native_ir::Expression::Nil => Ok(self.builder.ins().iconst(types::I64, NIL)),

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

            native_ir::Expression::IntAdd(left, right) => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;

                let fast = self.builder.create_block();
                let slow = self.builder.create_block();
                let join = self.builder.create_block();
                self.builder.append_block_param(join, types::I64);

                // Take the fast path only when both operands have their small
                // integer tag bit set.
                let both = self.builder.ins().band(left, right);
                let both_small = self.builder.ins().band_imm_u(both, 1);
                self.builder
                    .ins()
                    .brif(both_small, fast, &[], slow, &[]);
                self.builder.seal_block(fast);

                // (2x + 1) + (2y + 1) - 1 == 2(x + y) + 1, and overflow of
                // the tagged addition coincides with the mathematical result
                // leaving the small integer range.
                self.builder.switch_to_block(fast);
                let right_untagged = self.builder.ins().iadd_imm_s(right, -1);
                let (sum, overflowed) =
                    self.builder.ins().sadd_overflow(left, right_untagged);
                self.builder
                    .ins()
                    .brif(overflowed, slow, &[], join, &[sum.into()]);
                self.builder.seal_block(slow);

                self.builder.switch_to_block(slow);
                let slow_ref = self
                    .module
                    .declare_func_in_func(self.int_add_slow, self.builder.func);
                let call = self.builder.ins().call(slow_ref, &[left, right]);
                let slow_result = self.builder.inst_results(call)[0];
                self.builder.ins().jump(join, &[slow_result.into()]);
                self.builder.seal_block(join);

                self.builder.switch_to_block(join);
                Ok(self.builder.block_params(join)[0])
            }
        }
    }
}

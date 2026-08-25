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
use cranelift_codegen::ir::{
    AbiParam, Block, InstBuilder, MemFlagsData, Signature, TrapCode, Value, types,
};
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

/// The symbol of the runtime's string equality function.
pub const STRING_EQ: &str = "gleam_native_string_eq";

/// The symbols of the runtime's string prefix operations.
pub const STRING_STARTS_WITH: &str = "gleam_native_string_starts_with";
pub const STRING_SLICE_FROM: &str = "gleam_native_string_slice_from";

/// The symbols of the runtime's bit array operations.
pub const BITARRAY_EMPTY: &str = "gleam_native_bitarray_empty";
pub const BITARRAY_APPEND_INT: &str = "gleam_native_bitarray_append_int";
pub const BITARRAY_APPEND_FLOAT: &str = "gleam_native_bitarray_append_float";
pub const BITARRAY_APPEND_STRING: &str = "gleam_native_bitarray_append_string";
pub const BITARRAY_APPEND_CODEPOINT: &str = "gleam_native_bitarray_append_codepoint";
pub const BITARRAY_APPEND_BITS: &str = "gleam_native_bitarray_append_bits";
pub const BITARRAY_READ_FLOAT: &str = "gleam_native_bitarray_read_float";
pub const BITARRAY_IS_FINITE_FLOAT: &str = "gleam_native_bitarray_is_finite_float";
pub const BITARRAY_SIZE_TEST: &str = "gleam_native_bitarray_size_test";
pub const BITARRAY_BYTES_TEST: &str = "gleam_native_bitarray_bytes_test";
pub const BITARRAY_REST_IS_BYTES: &str = "gleam_native_bitarray_rest_is_bytes";
pub const BITARRAY_READ_INT: &str = "gleam_native_bitarray_read_int";
pub const BITARRAY_SLICE: &str = "gleam_native_bitarray_slice";

/// The symbol of the runtime's custom type record allocator.
pub const RECORD_NEW: &str = "gleam_native_record_new";

/// The symbol of the runtime's closure allocator.
pub const CLOSURE_NEW: &str = "gleam_native_closure_new";

/// The symbol of the runtime's structural deep equality.
pub const DEEP_EQ: &str = "gleam_native_eq";

/// The symbol of the runtime's `echo` implementation.
pub const ECHO: &str = "gleam_native_echo";

/// The symbols of the runtime's reference counting operations.
pub const INC: &str = "gleam_native_inc";
pub const DEC: &str = "gleam_native_dec";

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
    string_eq: FuncId,
    string_starts_with: FuncId,
    string_slice_from: FuncId,
    bitarray_empty: FuncId,
    bitarray_append_int: FuncId,
    bitarray_append_float: FuncId,
    bitarray_append_string: FuncId,
    bitarray_append_codepoint: FuncId,
    bitarray_append_bits: FuncId,
    bitarray_read_float: FuncId,
    bitarray_is_finite_float: FuncId,
    bitarray_size_test: FuncId,
    bitarray_bytes_test: FuncId,
    bitarray_rest_is_bytes: FuncId,
    bitarray_read_int: FuncId,
    bitarray_slice: FuncId,
    record_new: FuncId,
    closure_new: FuncId,
    deep_eq: FuncId,
    echo: FuncId,
    inc: FuncId,
    dec: FuncId,
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
            string_eq: declare(STRING_EQ, 2)?,
            string_starts_with: declare(STRING_STARTS_WITH, 3)?,
            string_slice_from: declare(STRING_SLICE_FROM, 2)?,
            bitarray_empty: declare(BITARRAY_EMPTY, 0)?,
            bitarray_append_int: declare(BITARRAY_APPEND_INT, 4)?,
            bitarray_append_float: declare(BITARRAY_APPEND_FLOAT, 4)?,
            bitarray_append_string: declare(BITARRAY_APPEND_STRING, 4)?,
            bitarray_append_codepoint: declare(BITARRAY_APPEND_CODEPOINT, 4)?,
            bitarray_append_bits: declare(BITARRAY_APPEND_BITS, 4)?,
            bitarray_read_float: declare(BITARRAY_READ_FLOAT, 4)?,
            bitarray_is_finite_float: declare(BITARRAY_IS_FINITE_FLOAT, 4)?,
            bitarray_size_test: declare(BITARRAY_SIZE_TEST, 3)?,
            bitarray_bytes_test: declare(BITARRAY_BYTES_TEST, 4)?,
            bitarray_rest_is_bytes: declare(BITARRAY_REST_IS_BYTES, 2)?,
            bitarray_read_int: declare(BITARRAY_READ_INT, 5)?,
            bitarray_slice: declare(BITARRAY_SLICE, 4)?,
            record_new: declare(RECORD_NEW, 3)?,
            closure_new: declare(CLOSURE_NEW, 2)?,
            deep_eq: declare(DEEP_EQ, 2)?,
            echo: declare(ECHO, 6)?,
            inc: declare(INC, 1)?,
            dec: declare(DEC, 1)?,
            panic: declare(PANIC, 7)?,
        })
    }
}

/// A function generated during translation whose body is defined after the
/// function that created it: a lifted lambda or a function-value wrapper.
enum PendingFunction {
    Lambda {
        id: FuncId,
        module_name: String,
        src_path: String,
        parameters: Vec<String>,
        captures: Vec<String>,
        body: Vec<native_ir::Statement>,
    },
    Wrapper {
        id: FuncId,
        target: FuncId,
        target_external: bool,
        arity: u32,
    },
}

pub struct Translator<'a, M: Module> {
    module: &'a mut M,
    /// Gleam (module, function) to declared Cranelift function and whether
    /// it is an external (C convention, borrows its arguments).
    functions: HashMap<(String, String), (FuncId, bool)>,
    runtime: RuntimeFunctions,
    pending: Vec<PendingFunction>,
    /// One wrapper per module function used as a value.
    wrappers: HashMap<(String, String), FuncId>,
    generated_counter: u32,
    /// Interned constructor names for `echo` display, id-indexed from
    /// `native_runtime::FIRST_INTERNED_DISPLAY`.
    display_names: Vec<String>,
    display_ids: HashMap<String, u16>,
}

impl<'a, M: Module> Translator<'a, M> {
    pub fn new(module: &'a mut M) -> Result<Self, String> {
        let runtime = RuntimeFunctions::declare(module)?;
        Ok(Self {
            module,
            functions: HashMap::new(),
            runtime,
            pending: Vec::new(),
            wrappers: HashMap::new(),
            generated_counter: 0,
            display_names: Vec::new(),
            display_ids: HashMap::new(),
        })
    }

    /// The interned constructor names collected during translation, for
    /// handing to the runtime.
    pub fn constructor_names(&self) -> Vec<String> {
        self.display_names.clone()
    }

    pub fn function_id(&self, module: &str, function: &str) -> Option<FuncId> {
        self.functions
            .get(&(module.to_string(), function.to_string()))
            .map(|(id, _)| *id)
    }

    /// First pass: declare every function of every module so that call sites
    /// can be resolved regardless of definition order.
    pub fn declare_module(&mut self, module: &native_ir::Module) -> Result<(), String> {
        let call_conv = self.module.isa().default_call_conv();
        for function in &module.functions {
            let (name, id, external) = match function {
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
                    (name, id, false)
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
                    (name, id, true)
                }
            };
            let _ = self
                .functions
                .insert((module.name.clone(), name.clone()), (id, external));
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
            self.define_function(id, &module.name, &module.src_path, parameters, body)?;
            self.define_pending()?;
        }
        Ok(())
    }

    /// Defines the bodies of functions generated during translation. Their
    /// bodies may generate further pending functions (nested lambdas).
    fn define_pending(&mut self) -> Result<(), String> {
        while let Some(pending) = self.pending.pop() {
            match pending {
                PendingFunction::Lambda {
                    id,
                    module_name,
                    src_path,
                    parameters,
                    captures,
                    body,
                } => {
                    self.define_lambda(id, &module_name, &src_path, &parameters, &captures, &body)?
                }
                PendingFunction::Wrapper {
                    id,
                    target,
                    target_external,
                    arity,
                } => self.define_wrapper(id, target, target_external, arity)?,
            }
        }
        Ok(())
    }

    /// Defines a lifted lambda: parameters arrive after the closure
    /// argument, and captures load out of the closure.
    fn define_lambda(
        &mut self,
        id: FuncId,
        module_name: &str,
        src_path: &str,
        parameters: &[String],
        captures: &[String],
        body: &[native_ir::Statement],
    ) -> Result<(), String> {
        let mut context = self.module.make_context();
        context.func.signature = gleam_signature(1 + parameters.len());

        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let closure = builder.block_params(entry)[0];
        let mut environment = HashMap::new();
        let mut scope_owned = Vec::new();
        for (index, parameter) in parameters.iter().enumerate() {
            let value = builder.block_params(entry)[1 + index];
            let variable = builder.declare_var(types::I64);
            builder.def_var(variable, value);
            let _ = environment.insert(parameter.clone(), variable);
            scope_owned.push((parameter.clone(), variable));
        }
        for (index, capture) in captures.iter().enumerate() {
            let value = builder.ins().load(
                types::I64,
                MemFlagsData::trusted(),
                closure,
                16 + 8 * index as i32,
            );
            let variable = builder.declare_var(types::I64);
            builder.def_var(variable, value);
            let _ = environment.insert(capture.clone(), variable);
            scope_owned.push((capture.clone(), variable));
        }

        let mut function_translator = FunctionTranslator {
            functions: &self.functions,
            runtime: self.runtime,
            module_name,
            src_path,
            module: self.module,
            builder: &mut builder,
            environment,
            pending: &mut self.pending,
            wrappers: &mut self.wrappers,
            generated_counter: &mut self.generated_counter,
            display_names: &mut self.display_names,
            display_ids: &mut self.display_ids,
            scope_owned,
        };
        // The capture slots take their own references, then the closure
        // itself (owned by this call) is released.
        for index in 0..captures.len() {
            let slot = function_translator.scope_owned[parameters.len() + index].1;
            let value = function_translator.builder.use_var(slot);
            let _ = function_translator.inc(value);
        }
        function_translator.dec(closure);
        if let Some(result) =
            function_translator.statements_scoped(body, 0, Some(Vec::new()))?
        {
            builder.ins().return_(&[result]);
        }
        builder.finalize(self.module.target_config());

        self.module
            .define_function(id, &mut context)
            .map_err(|error| error.to_string())?;
        self.module.clear_context(&mut context);
        Ok(())
    }

    /// Defines the wrapper that adapts a directly-callable function to the
    /// closure calling convention: release the closure, forward the rest.
    fn define_wrapper(
        &mut self,
        id: FuncId,
        target: FuncId,
        target_external: bool,
        arity: u32,
    ) -> Result<(), String> {
        let mut context = self.module.make_context();
        context.func.signature = gleam_signature(1 + arity as usize);

        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let closure = builder.block_params(entry)[0];
        let arguments: Vec<Value> = builder.block_params(entry)[1..].to_vec();
        let dec_ref = self
            .module
            .declare_func_in_func(self.runtime.dec, builder.func);
        let target_ref = self.module.declare_func_in_func(target, builder.func);
        if target_external {
            // Externals borrow: call, then release what this wrapper owns.
            let call = builder.ins().call(target_ref, &arguments);
            let result = builder.inst_results(call)[0];
            let _ = builder.ins().call(dec_ref, &[closure]);
            for argument in &arguments {
                let _ = builder.ins().call(dec_ref, &[*argument]);
            }
            builder.ins().return_(&[result]);
        } else {
            // Gleam functions own their arguments: release the closure and
            // transfer the rest with a genuine tail call.
            let _ = builder.ins().call(dec_ref, &[closure]);
            builder.ins().return_call(target_ref, &arguments);
        }
        builder.finalize(self.module.target_config());

        self.module
            .define_function(id, &mut context)
            .map_err(|error| error.to_string())?;
        self.module.clear_context(&mut context);
        Ok(())
    }

    fn define_function(
        &mut self,
        id: FuncId,
        module_name: &str,
        src_path: &str,
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
        let mut scope_owned = Vec::new();
        for (index, parameter) in parameters.iter().enumerate() {
            let value = builder.block_params(entry)[index];
            let variable = builder.declare_var(types::I64);
            builder.def_var(variable, value);
            let _ = environment.insert(parameter.clone(), variable);
            // The callee owns its arguments; parameters are released like
            // any other scope binding (or early, when dead).
            scope_owned.push((parameter.clone(), variable));
        }

        let mut function_translator = FunctionTranslator {
            functions: &self.functions,
            runtime: self.runtime,
            module_name,
            src_path,
            module: self.module,
            builder: &mut builder,
            environment,
            pending: &mut self.pending,
            wrappers: &mut self.wrappers,
            generated_counter: &mut self.generated_counter,
            display_names: &mut self.display_names,
            display_ids: &mut self.display_ids,
            scope_owned,
        };
        if let Some(result) =
            function_translator.statements_scoped(body, 0, Some(Vec::new()))?
        {
            builder.ins().return_(&[result]);
        }
        builder.finalize(self.module.target_config());

        self.module
            .define_function(id, &mut context)
            .map_err(|error| error.to_string())?;
        self.module.clear_context(&mut context);
        Ok(())
    }

    /// Generates the C-convention wrapper the host uses to call `main`.
    pub fn define_entry_wrapper(&mut self, main: FuncId) -> Result<FuncId, String> {
        self.define_c_wrapper(ENTRY_SYMBOL, main)
    }

    /// Generates a C-convention, zero-argument wrapper around the given
    /// function, under the given symbol. The test runner uses one per test
    /// so the host can call compiled tests directly.
    pub fn define_c_wrapper(&mut self, symbol: &str, target: FuncId) -> Result<FuncId, String> {
        let call_conv = self.module.isa().default_call_conv();
        let signature = c_signature(call_conv, 0);
        let id = self
            .module
            .declare_function(symbol, Linkage::Export, &signature)
            .map_err(|error| error.to_string())?;

        let mut context = self.module.make_context();
        context.func.signature = signature;
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let target_ref = self.module.declare_func_in_func(target, builder.func);
        let call = builder.ins().call(target_ref, &[]);
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

/// How a decision tree's leaves behave: `case` clause bodies run in their
/// own scope, while assignment destructuring binds into the enclosing scope
/// and yields the subject value.
#[derive(Clone)]
enum DecisionMode<'a> {
    Case {
        /// In tail position: the owned values (enclosing scopes, subjects)
        /// to release before a tail transfer inside a clause.
        tail: Option<Vec<Value>>,
    },
    Assignment {
        result: Value,
        on_failure: Option<&'a native_ir::AssignmentFailure>,
    },
}

struct FunctionTranslator<'a, 'b, M: Module> {
    functions: &'a HashMap<(String, String), (FuncId, bool)>,
    runtime: RuntimeFunctions,
    module_name: &'a str,
    /// The module's package-root-relative source path, printed by `echo`.
    src_path: &'a str,
    module: &'a mut M,
    builder: &'a mut FunctionBuilder<'b>,
    environment: HashMap<String, Variable>,
    pending: &'a mut Vec<PendingFunction>,
    wrappers: &'a mut HashMap<(String, String), FuncId>,
    generated_counter: &'a mut u32,
    display_names: &'a mut Vec<String>,
    display_ids: &'a mut HashMap<String, u16>,
    /// Named variable slots holding owned references, decremented when
    /// their scope's statement sequence finishes — or just before its final
    /// statement, for bindings the final statement does not mention, so that
    /// tail-recursive loops release their garbage every iteration.
    scope_owned: Vec<(String, Variable)>,
}

impl<M: Module> FunctionTranslator<'_, '_, M> {
    /// Allocates a closure: one word for the code pointer followed by the
    /// captured values, read out of the current environment. The arity is
    /// the function's parameter count without the closure argument, stored
    /// for `echo`'s function rendering.
    fn make_closure(
        &mut self,
        function: FuncId,
        captures: &[String],
        arity: u32,
    ) -> Result<Value, String> {
        let capture_count = self.builder.ins().iconst(types::I64, captures.len() as i64);
        let arity = self.builder.ins().iconst(types::I64, arity as i64);
        let closure_new_ref = self
            .module
            .declare_func_in_func(self.runtime.closure_new, self.builder.func);
        let call = self
            .builder
            .ins()
            .call(closure_new_ref, &[capture_count, arity]);
        let closure = self.builder.inst_results(call)[0];

        let function_ref = self.module.declare_func_in_func(function, self.builder.func);
        let pointer_type = self.module.target_config().pointer_type();
        let address = self.builder.ins().func_addr(pointer_type, function_ref);
        let _ = self
            .builder
            .ins()
            .store(MemFlagsData::trusted(), address, closure, 8);

        for (index, name) in captures.iter().enumerate() {
            let variable = self
                .environment
                .get(name)
                .ok_or_else(|| format!("unbound capture `{name}`"))?;
            let value = self.builder.use_var(*variable);
            let _ = self.inc(value);
            let _ = self.builder.ins().store(
                MemFlagsData::trusted(),
                value,
                closure,
                16 + 8 * index as i32,
            );
        }
        Ok(closure)
    }

    /// Emits a call to the runtime's report-and-abort function. It never
    /// actually returns; treating it as an ordinary call keeps the block
    /// structure simple, and the code after it is simply never reached.
    fn emit_panic(
        &mut self,
        kind: i64,
        message: Option<&native_ir::Expression>,
        function: &str,
        line: u32,
    ) -> Result<Value, String> {
        let kind = self.builder.ins().iconst(types::I64, kind);
        let message = match message {
            Some(message) => self.expression(message)?,
            None => self.builder.ins().iconst(types::I64, 0),
        };
        let module_name = self.module_name.to_string();
        let (module_pointer, module_length) = self.constant_bytes(module_name.as_bytes())?;
        let (function_pointer, function_length) =
            self.constant_bytes(function.to_string().as_bytes())?;
        let line = self.builder.ins().iconst(types::I64, line as i64);
        let panic_ref = self
            .module
            .declare_func_in_func(self.runtime.panic, self.builder.func);
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
    /// The runtime flag for an endianness: 0 big, 1 little, 2 native.
    fn endian_flag(&mut self, endian: native_ir::Endian) -> Value {
        self.builder.ins().iconst(
            types::I64,
            match endian {
                native_ir::Endian::Big => 0,
                native_ir::Endian::Little => 1,
                native_ir::Endian::Native => 2,
            },
        )
    }

    /// The runtime flag for a string encoding: 0 utf8, 1 utf16, 2 utf32.
    fn encoding_flag(&mut self, encoding: native_ir::StringEncoding) -> Value {
        self.builder.ins().iconst(
            types::I64,
            match encoding {
                native_ir::StringEncoding::Utf8 => 0,
                native_ir::StringEncoding::Utf16 => 1,
                native_ir::StringEncoding::Utf32 => 2,
            },
        )
    }

    /// Emits an integer read out of a bit array with expression-valued
    /// offset and size (both tagged).
    fn emit_bits_read(
        &mut self,
        subject: Value,
        read: &native_ir::SegmentRead,
    ) -> Result<Value, String> {
        let offset = self.expression(&read.offset)?;
        let bits = self.expression(&read.bits)?;
        let endian = self.endian_flag(read.endian);
        let signed = self.builder.ins().iconst(types::I64, read.signed as i64);
        let read_ref = self
            .module
            .declare_func_in_func(self.runtime.bitarray_read_int, self.builder.func);
        let call = self
            .builder
            .ins()
            .call(read_ref, &[subject, offset, bits, endian, signed]);
        let result = self.builder.inst_results(call)[0];
        self.dec(offset);
        self.dec(bits);
        Ok(result)
    }

    /// Emits a reference count increment. Yields the value for chaining.
    fn inc(&mut self, value: Value) -> Value {
        let inc_ref = self
            .module
            .declare_func_in_func(self.runtime.inc, self.builder.func);
        let _ = self.builder.ins().call(inc_ref, &[value]);
        value
    }

    /// Emits a reference count decrement.
    fn dec(&mut self, value: Value) {
        let dec_ref = self
            .module
            .declare_func_in_func(self.runtime.dec, self.builder.func);
        let _ = self.builder.ins().call(dec_ref, &[value]);
    }

    /// Translates a statement sequence with ownership bookkeeping: values of
    /// discarded statements are decremented immediately, `let`-bound slots
    /// are decremented when the sequence finishes, and a final `let`'s value
    /// is incremented since both the binding and the sequence result own it.
    fn statements(&mut self, statements: &[native_ir::Statement]) -> Result<Value, String> {
        let scope_start = self.scope_owned.len();
        match self.statements_scoped(statements, scope_start, None)? {
            Some(value) => Ok(value),
            None => unreachable!("tail transfer outside a tail context"),
        }
    }

    /// Like [`Self::statements`], but with an explicit scope start (so
    /// function parameters participate in scope release) and, when in tail
    /// position, the owned values of enclosing scopes to release before a
    /// tail transfer. Returns `None` when the sequence ended in a genuine
    /// tail call and control never returns here.
    fn statements_scoped(
        &mut self,
        statements: &[native_ir::Statement],
        scope_start: usize,
        tail: Option<Vec<Value>>,
    ) -> Result<Option<Value>, String> {
        let mut result = None;
        for (index, statement) in statements.iter().enumerate() {
            let is_final = index + 1 == statements.len();
            if is_final {
                // Release bindings the final statement does not mention now,
                // so a tail call does not keep them alive down the recursion.
                let free = native_ir::free_variables(
                    std::slice::from_ref(statement),
                    &std::collections::HashSet::new(),
                );
                let kept = self.scope_owned.split_off(scope_start);
                for (name, slot) in kept {
                    if free.contains(&name) {
                        self.scope_owned.push((name, slot));
                    } else {
                        let value = self.builder.use_var(slot);
                        self.dec(value);
                    }
                }
                // In tail position, a final call transfers control: hand it
                // the values still owed a release.
                if let (Some(outer), native_ir::Statement::Expression(expression)) =
                    (&tail, statement)
                {
                    let mut cleanups = outer.clone();
                    for (_, slot) in &self.scope_owned[scope_start..] {
                        cleanups.push(self.builder.use_var(*slot));
                    }
                    match self.expression_tail(expression, cleanups)? {
                        None => {
                            // Transferred; only the bookkeeping remains.
                            self.scope_owned.truncate(scope_start);
                            return Ok(None);
                        }
                        Some(value) => {
                            result = Some(value);
                            break;
                        }
                    }
                }
            }
            result = Some(match statement {
                native_ir::Statement::Expression(expression) => {
                    let value = self.expression(expression)?;
                    if !is_final {
                        self.dec(value);
                    }
                    value
                }
                native_ir::Statement::Let { name, value } => {
                    let value = self.expression(value)?;
                    let variable = self.builder.declare_var(types::I64);
                    self.builder.def_var(variable, value);
                    let _ = self.environment.insert(name.clone(), variable);
                    self.scope_owned.push((name.clone(), variable));
                    if is_final {
                        let _ = self.inc(value);
                    }
                    value
                }
                native_ir::Statement::Destructure {
                    subject,
                    subject_id,
                    tree,
                    on_failure,
                } => {
                    let value = self.expression(subject)?;
                    let mut variables = HashMap::new();
                    let _ = variables.insert(*subject_id, value);

                    let join = self.builder.create_block();
                    self.builder.append_block_param(join, types::I64);
                    let _ = self.decision(
                        &variables,
                        tree,
                        join,
                        DecisionMode::Assignment {
                            result: value,
                            on_failure: on_failure.as_ref(),
                        },
                    )?;
                    self.builder.seal_block(join);
                    self.builder.switch_to_block(join);
                    let result = self.builder.block_params(join)[0];
                    if !is_final {
                        self.dec(result);
                    }
                    result
                }
            });
        }
        let result = match result {
            Some(value) => value,
            None => self.builder.ins().iconst(types::I64, NIL),
        };
        // Release everything this scope's bindings still own.
        while self.scope_owned.len() > scope_start {
            let (_, slot) = self.scope_owned.pop().expect("scope slot");
            let value = self.builder.use_var(slot);
            self.dec(value);
        }
        Ok(Some(result))
    }

    /// Translates an expression in tail position: a call becomes a genuine
    /// tail transfer (releasing `cleanups` between argument evaluation and
    /// the transfer), and blocks and case expressions propagate tailness
    /// inwards. Returns `None` when control was transferred.
    fn expression_tail(
        &mut self,
        expression: &native_ir::Expression,
        cleanups: Vec<Value>,
    ) -> Result<Option<Value>, String> {
        match expression {
            native_ir::Expression::Call {
                module,
                function,
                arguments,
            } => {
                let (id, external) = *self
                    .functions
                    .get(&(module.clone(), function.clone()))
                    .ok_or_else(|| format!("unknown function `{module}.{function}`"))?;
                if external {
                    // C-convention functions cannot be tail called.
                    return Ok(Some(self.expression(expression)?));
                }
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                for cleanup in cleanups {
                    self.dec(cleanup);
                }
                let function_ref = self.module.declare_func_in_func(id, self.builder.func);
                self.builder.ins().return_call(function_ref, &values);
                Ok(None)
            }

            native_ir::Expression::CallValue { callee, arguments } => {
                let callee = self.expression(callee)?;
                let mut values = Vec::with_capacity(1 + arguments.len());
                values.push(callee);
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                let code = self
                    .builder
                    .ins()
                    .load(types::I64, MemFlagsData::trusted(), callee, 8);
                for cleanup in cleanups {
                    self.dec(cleanup);
                }
                let signature = self.builder.import_signature(gleam_signature(values.len()));
                self.builder
                    .ins()
                    .return_call_indirect(signature, code, &values);
                Ok(None)
            }

            native_ir::Expression::Block(statements) => {
                let scope_start = self.scope_owned.len();
                self.statements_scoped(statements, scope_start, Some(cleanups))
            }

            native_ir::Expression::Case {
                subjects,
                subject_ids,
                tree,
            } => {
                let mut variables = HashMap::new();
                let mut subject_values = Vec::with_capacity(subjects.len());
                for (id, subject) in subject_ids.iter().zip(subjects) {
                    let value = self.expression(subject)?;
                    let _ = variables.insert(*id, value);
                    subject_values.push(value);
                }
                let mut tail_cleanups = cleanups;
                tail_cleanups.extend(subject_values.iter().copied());

                let join = self.builder.create_block();
                self.builder.append_block_param(join, types::I64);
                let joined = self.decision(
                    &variables,
                    tree,
                    join,
                    DecisionMode::Case {
                        tail: Some(tail_cleanups),
                    },
                )?;
                self.builder.seal_block(join);
                self.builder.switch_to_block(join);
                if joined {
                    let result = self.builder.block_params(join)[0];
                    for subject in subject_values {
                        self.dec(subject);
                    }
                    Ok(Some(result))
                } else {
                    // Every branch transferred; the join is unreachable.
                    self.builder
                        .ins()
                        .trap(TrapCode::user(1).expect("valid trap code"));
                    Ok(None)
                }
            }

            _ => Ok(Some(self.expression(expression)?)),
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

            // The empty list is the tagged small integer 0; cons cells are
            // heap records, so any pointer-valued list is non-empty.
            native_ir::Expression::EmptyList => {
                Ok(self.builder.ins().iconst(types::I64, NIL))
            }

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
                let value = self.builder.use_var(*variable);
                Ok(self.inc(value))
            }

            native_ir::Expression::Block(statements) => self.statements(statements),

            native_ir::Expression::Call {
                module,
                function,
                arguments,
            } => {
                let (id, external) = *self
                    .functions
                    .get(&(module.clone(), function.clone()))
                    .ok_or_else(|| format!("unknown function `{module}.{function}`"))?;
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                let function_ref = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(function_ref, &values);
                let result = self.builder.inst_results(call)[0];
                // Gleam callees own their arguments; externals only borrow.
                if external {
                    for value in values {
                        self.dec(value);
                    }
                }
                Ok(result)
            }

            native_ir::Expression::StringConcat(left, right) => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let concat_ref = self
                    .module
                    .declare_func_in_func(self.runtime.string_concat, self.builder.func);
                let call = self.builder.ins().call(concat_ref, &[left, right]);
                let result = self.builder.inst_results(call)[0];
                self.dec(left);
                self.dec(right);
                Ok(result)
            }

            native_ir::Expression::Case {
                subjects,
                subject_ids,
                tree,
            } => {
                let mut variables = HashMap::new();
                for (id, subject) in subject_ids.iter().zip(subjects) {
                    let value = self.expression(subject)?;
                    let _ = variables.insert(*id, value);
                }

                let join = self.builder.create_block();
                self.builder.append_block_param(join, types::I64);
                let _ = self.decision(&variables, tree, join, DecisionMode::Case { tail: None })?;
                self.builder.seal_block(join);

                self.builder.switch_to_block(join);
                let result = self.builder.block_params(join)[0];
                for subject in variables.values() {
                    self.dec(*subject);
                }
                Ok(result)
            }

            native_ir::Expression::Constructor {
                tag,
                display,
                arguments,
            } => {
                let display = match display {
                    native_ir::ConstructorDisplay::Tuple => native_runtime::DISPLAY_TUPLE,
                    native_ir::ConstructorDisplay::List => native_runtime::DISPLAY_LIST,
                    native_ir::ConstructorDisplay::Record { name } => match name.as_str() {
                        "Ok" => native_runtime::DISPLAY_OK,
                        "Error" => native_runtime::DISPLAY_ERROR,
                        "Some" => native_runtime::DISPLAY_SOME,
                        "None" => native_runtime::DISPLAY_NONE,
                        name => match self.display_ids.get(name) {
                            Some(id) => *id,
                            None => {
                                let id = native_runtime::FIRST_INTERNED_DISPLAY
                                    + self.display_names.len() as u16;
                                self.display_names.push(name.to_string());
                                let _ = self.display_ids.insert(name.to_string(), id);
                                id
                            }
                        },
                    },
                };
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                let tag = self.builder.ins().iconst(types::I64, *tag as i64);
                let arity = self.builder.ins().iconst(types::I64, values.len() as i64);
                let display = self.builder.ins().iconst(types::I64, display as i64);
                let record_new_ref = self
                    .module
                    .declare_func_in_func(self.runtime.record_new, self.builder.func);
                let call = self
                    .builder
                    .ins()
                    .call(record_new_ref, &[tag, arity, display]);
                let record = self.builder.inst_results(call)[0];
                for (index, value) in values.into_iter().enumerate() {
                    let _ = self.builder.ins().store(
                        MemFlagsData::trusted(),
                        value,
                        record,
                        8 + 8 * index as i32,
                    );
                }
                Ok(record)
            }

            native_ir::Expression::FieldAccess { record, index } => {
                let record = self.expression(record)?;
                let field = self.builder.ins().load(
                    types::I64,
                    MemFlagsData::trusted(),
                    record,
                    8 + 8 * *index as i32,
                );
                let _ = self.inc(field);
                self.dec(record);
                Ok(field)
            }

            native_ir::Expression::Lambda { parameters, body } => {
                // Captures: the lambda's free variables that are in scope
                // here. A missed one fails loudly when the lifted body is
                // defined; extras would be harmless.
                let bound: std::collections::HashSet<String> =
                    parameters.iter().cloned().collect();
                let captures: Vec<String> = native_ir::free_variables(body, &bound)
                    .into_iter()
                    .filter(|name| self.environment.contains_key(name))
                    .collect();

                *self.generated_counter += 1;
                let symbol = format!("gleam$lambda${}", self.generated_counter);
                let id = self
                    .module
                    .declare_function(
                        &symbol,
                        Linkage::Export,
                        &gleam_signature(1 + parameters.len()),
                    )
                    .map_err(|error| error.to_string())?;
                self.pending.push(PendingFunction::Lambda {
                    id,
                    module_name: self.module_name.to_string(),
                    src_path: self.src_path.to_string(),
                    parameters: parameters.clone(),
                    captures: captures.clone(),
                    body: body.clone(),
                });
                self.make_closure(id, &captures, parameters.len() as u32)
            }

            native_ir::Expression::FunctionReference {
                module,
                function,
                arity,
            } => {
                let (target, target_external) = *self
                    .functions
                    .get(&(module.clone(), function.clone()))
                    .ok_or_else(|| format!("unknown function `{module}.{function}`"))?;
                let key = (module.clone(), function.clone());
                let id = match self.wrappers.get(&key) {
                    Some(id) => *id,
                    None => {
                        *self.generated_counter += 1;
                        let symbol = format!("gleam$wrapper${}", self.generated_counter);
                        let id = self
                            .module
                            .declare_function(
                                &symbol,
                                Linkage::Export,
                                &gleam_signature(1 + *arity as usize),
                            )
                            .map_err(|error| error.to_string())?;
                        self.pending.push(PendingFunction::Wrapper {
                            id,
                            target,
                            target_external,
                            arity: *arity,
                        });
                        let _ = self.wrappers.insert(key, id);
                        id
                    }
                };
                self.make_closure(id, &[], *arity)
            }

            native_ir::Expression::CallValue { callee, arguments } => {
                let callee = self.expression(callee)?;
                let mut values = Vec::with_capacity(1 + arguments.len());
                values.push(callee);
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                let code = self
                    .builder
                    .ins()
                    .load(types::I64, MemFlagsData::trusted(), callee, 8);
                let signature = self
                    .builder
                    .import_signature(gleam_signature(values.len()));
                let call = self.builder.ins().call_indirect(signature, code, &values);
                let result = self.builder.inst_results(call)[0];
                // The callee owns the closure and the arguments.
                Ok(result)
            }

            native_ir::Expression::BitArray(segments) => {
                let empty_ref = self
                    .module
                    .declare_func_in_func(self.runtime.bitarray_empty, self.builder.func);
                let call = self.builder.ins().call(empty_ref, &[]);
                let mut array = self.builder.inst_results(call)[0];
                for segment in segments {
                    let value = self.expression(&segment.value)?;
                    array = match &segment.kind {
                        native_ir::BitSegmentKind::Int { bits, endian } => {
                            let bits = self.expression(bits)?;
                            let endian = self.endian_flag(*endian);
                            let append_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_append_int,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(append_ref, &[array, value, bits, endian]);
                            let result = self.builder.inst_results(call)[0];
                            self.dec(bits);
                            result
                        }
                        native_ir::BitSegmentKind::Float { bits, endian } => {
                            let bits = self.expression(bits)?;
                            let endian = self.endian_flag(*endian);
                            let append_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_append_float,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(append_ref, &[array, value, bits, endian]);
                            let result = self.builder.inst_results(call)[0];
                            self.dec(bits);
                            result
                        }
                        native_ir::BitSegmentKind::String { encoding, endian } => {
                            let encoding = self.encoding_flag(*encoding);
                            let endian = self.endian_flag(*endian);
                            let append_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_append_string,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(append_ref, &[array, value, encoding, endian]);
                            self.builder.inst_results(call)[0]
                        }
                        native_ir::BitSegmentKind::Codepoint { encoding, endian } => {
                            let encoding = self.encoding_flag(*encoding);
                            let endian = self.endian_flag(*endian);
                            let append_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_append_codepoint,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(append_ref, &[array, value, encoding, endian]);
                            self.builder.inst_results(call)[0]
                        }
                        native_ir::BitSegmentKind::BitArraySplice { bits } => {
                            let (bits, has_bits) = match bits {
                                Some(bits) => {
                                    let bits = self.expression(bits)?;
                                    let one = self.builder.ins().iconst(types::I64, 1);
                                    (bits, one)
                                }
                                None => {
                                    let placeholder = self.builder.ins().iconst(types::I64, 1);
                                    let zero = self.builder.ins().iconst(types::I64, 0);
                                    (placeholder, zero)
                                }
                            };
                            let append_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_append_bits,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(append_ref, &[array, value, bits, has_bits]);
                            self.builder.inst_results(call)[0]
                        }
                    };
                    self.dec(value);
                }
                Ok(array)
            }

            native_ir::Expression::Echo {
                kind,
                value,
                message,
                line,
            } => {
                let kind = self.builder.ins().iconst(
                    types::I64,
                    match kind {
                        native_ir::EchoKind::Structural => 0,
                        native_ir::EchoKind::Int => 1,
                        native_ir::EchoKind::Float => 2,
                        native_ir::EchoKind::String => 3,
                        native_ir::EchoKind::Bool => 4,
                        native_ir::EchoKind::Nil => 5,
                        native_ir::EchoKind::List => 6,
                    },
                );
                let value = self.expression(value)?;
                let (message, message_owned) = match message {
                    Some(message) => (self.expression(message)?, true),
                    None => (self.builder.ins().iconst(types::I64, 0), false),
                };
                let src_path = self.src_path.to_string();
                let (module_pointer, module_length) =
                    self.constant_bytes(src_path.as_bytes())?;
                let line = self.builder.ins().iconst(types::I64, *line as i64);
                let echo_ref = self
                    .module
                    .declare_func_in_func(self.runtime.echo, self.builder.func);
                let call = self.builder.ins().call(
                    echo_ref,
                    &[kind, value, message, module_pointer, module_length, line],
                );
                let result = self.builder.inst_results(call)[0];
                if message_owned {
                    self.dec(message);
                }
                Ok(result)
            }

            native_ir::Expression::Panic {
                kind,
                message,
                function,
                line,
            } => {
                let kind = match kind {
                    native_ir::PanicKind::Panic => 0,
                    native_ir::PanicKind::Todo => 1,
                    native_ir::PanicKind::LetAssert => 2,
                    native_ir::PanicKind::Assert => 3,
                };
                let function = function.clone();
                self.emit_panic(kind, message.as_deref(), &function, *line)
            }

            native_ir::Expression::IntBinary {
                operator,
                left,
                right,
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let result = self.int_binary(*operator, left, right)?;
                self.dec(left);
                self.dec(right);
                Ok(result)
            }

            native_ir::Expression::IntCompare {
                operator,
                left,
                right,
            } => {
                let condition = match operator {
                    native_ir::CompareOperator::LessThan => IntCC::SignedLessThan,
                    native_ir::CompareOperator::LessThanOrEqual => IntCC::SignedLessThanOrEqual,
                    native_ir::CompareOperator::GreaterThan => IntCC::SignedGreaterThan,
                    native_ir::CompareOperator::GreaterThanOrEqual => {
                        IntCC::SignedGreaterThanOrEqual
                    }
                };
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let result = self.int_compare(condition, left, right)?;
                self.dec(left);
                self.dec(right);
                Ok(result)
            }

            native_ir::Expression::Equality {
                kind,
                negated,
                left,
                right,
            } => {
                let left = self.expression(left)?;
                let right = self.expression(right)?;
                let result = match kind {
                    // Bool and Nil are always tagged immediates: word
                    // equality is value equality.
                    native_ir::EqualityKind::Immediate => {
                        let condition = if *negated {
                            IntCC::NotEqual
                        } else {
                            IntCC::Equal
                        };
                        let flag = self.builder.ins().icmp(condition, left, right);
                        Ok(self.tag_boolean_flag(flag))
                    }
                    // Big integers never encode values in the small range,
                    // so the ordinary comparison diamond is exact for
                    // equality too.
                    native_ir::EqualityKind::Int => {
                        let condition = if *negated {
                            IntCC::NotEqual
                        } else {
                            IntCC::Equal
                        };
                        self.int_compare(condition, left, right)
                    }
                    native_ir::EqualityKind::Float => {
                        let condition = if *negated {
                            FloatCC::NotEqual
                        } else {
                            FloatCC::Equal
                        };
                        let left = self.load_float(left);
                        let right = self.load_float(right);
                        let flag = self.builder.ins().fcmp(condition, left, right);
                        Ok(self.tag_boolean_flag(flag))
                    }
                    native_ir::EqualityKind::Deep => {
                        let eq_ref = self
                            .module
                            .declare_func_in_func(self.runtime.deep_eq, self.builder.func);
                        let call = self.builder.ins().call(eq_ref, &[left, right]);
                        let result = self.builder.inst_results(call)[0];
                        if *negated {
                            // Flip between the tagged booleans 1 and 3.
                            Ok(self.builder.ins().bxor_imm_u(result, 2))
                        } else {
                            Ok(result)
                        }
                    }
                    native_ir::EqualityKind::String => {
                        let eq_ref = self
                            .module
                            .declare_func_in_func(self.runtime.string_eq, self.builder.func);
                        let call = self.builder.ins().call(eq_ref, &[left, right]);
                        let result = self.builder.inst_results(call)[0];
                        if *negated {
                            // Flip between the tagged booleans 1 and 3.
                            Ok(self.builder.ins().bxor_imm_u(result, 2))
                        } else {
                            Ok(result)
                        }
                    }
                }?;
                self.dec(left);
                self.dec(right);
                Ok(result)
            }

            native_ir::Expression::BoolNot(expression) => {
                let value = self.expression(expression)?;
                // Flip between the tagged booleans 1 and 3.
                Ok(self.builder.ins().bxor_imm_u(value, 2))
            }

            native_ir::Expression::BoolBinary {
                operator,
                left,
                right,
            } => {
                let left = self.expression(left)?;

                let evaluate_right = self.builder.create_block();
                let join = self.builder.create_block();
                self.builder.append_block_param(join, types::I64);

                // Booleans are the tagged small integers 1 (False) and
                // 3 (True), so bit 1 distinguishes them. When the left value
                // decides the result it flows to the join unchanged.
                let is_true = self.builder.ins().band_imm_u(left, 2);
                match operator {
                    native_ir::BoolOperator::And => {
                        self.builder
                            .ins()
                            .brif(is_true, evaluate_right, &[], join, &[left.into()]);
                    }
                    native_ir::BoolOperator::Or => {
                        self.builder
                            .ins()
                            .brif(is_true, join, &[left.into()], evaluate_right, &[]);
                    }
                }
                self.builder.seal_block(evaluate_right);

                self.builder.switch_to_block(evaluate_right);
                let right = self.expression(right)?;
                self.builder.ins().jump(join, &[right.into()]);
                self.builder.seal_block(join);

                self.builder.switch_to_block(join);
                Ok(self.builder.block_params(join)[0])
            }

            native_ir::Expression::FloatBinary {
                operator,
                left,
                right,
            } => {
                let left_boxed = self.expression(left)?;
                let right_boxed = self.expression(right)?;
                let left = self.load_float(left_boxed);
                let right = self.load_float(right_boxed);
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
                let result = self.box_float(result)?;
                self.dec(left_boxed);
                self.dec(right_boxed);
                Ok(result)
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
                let left_boxed = self.expression(left)?;
                let right_boxed = self.expression(right)?;
                let left = self.load_float(left_boxed);
                let right = self.load_float(right_boxed);
                let flag = self.builder.ins().fcmp(condition, left, right);
                let result = self.tag_boolean_flag(flag);
                self.dec(left_boxed);
                self.dec(right_boxed);
                Ok(result)
            }
        }
    }

    /// Emits a decision tree node into the current block. Every path through
    /// the tree either jumps to `join` with the matched clause's result or
    /// traps on the unreachable `Fail` node.
    fn decision(
        &mut self,
        variables: &HashMap<u32, Value>,
        tree: &native_ir::Decision,
        join: Block,
        mode: DecisionMode<'_>,
    ) -> Result<bool, String> {
        match tree {
            native_ir::Decision::Run { bindings, body } => {
                // In a case expression, clause bindings must not leak past
                // this clause. In an assignment they are the whole point and
                // persist in the enclosing scope.
                let saved_environment = match mode {
                    DecisionMode::Case { .. } => Some(self.environment.clone()),
                    DecisionMode::Assignment { .. } => None,
                };
                let mut binding_slots = Vec::with_capacity(bindings.len());
                for (name, bound) in bindings {
                    let value = match bound {
                        native_ir::Bound::Variable(id) => {
                            let value = *variables
                                .get(id)
                                .ok_or_else(|| format!("unbound decision variable {id}"))?;
                            // The binding shares the container's reference.
                            self.inc(value)
                        }
                        native_ir::Bound::Value(expression) => self.expression(expression)?,
                        native_ir::Bound::StringSlice { subject, offset } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let offset = self.builder.ins().iconst(types::I64, *offset as i64);
                            let slice_ref = self.module.declare_func_in_func(
                                self.runtime.string_slice_from,
                                self.builder.func,
                            );
                            let call = self.builder.ins().call(slice_ref, &[subject, offset]);
                            self.builder.inst_results(call)[0]
                        }
                        native_ir::Bound::BitsReadInt {
                            subject,
                            offset,
                            bits,
                            endian,
                            signed,
                        } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let read = native_ir::SegmentRead {
                                name: String::new(),
                                offset: offset.clone(),
                                bits: bits.clone(),
                                endian: *endian,
                                signed: *signed,
                            };
                            self.emit_bits_read(subject, &read)?
                        }
                        native_ir::Bound::BitsReadFloat {
                            subject,
                            offset,
                            bits,
                            endian,
                        } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let offset = self.expression(offset)?;
                            let bits = self.expression(bits)?;
                            let endian = self.endian_flag(*endian);
                            let read_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_read_float,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(read_ref, &[subject, offset, bits, endian]);
                            let result = self.builder.inst_results(call)[0];
                            self.dec(offset);
                            self.dec(bits);
                            result
                        }
                        native_ir::Bound::BitsSlice {
                            subject,
                            offset,
                            bits,
                        } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let offset = self.expression(offset)?;
                            let (bits, has_bits) = match bits {
                                Some(bits) => {
                                    let bits = self.expression(bits)?;
                                    let one = self.builder.ins().iconst(types::I64, 1);
                                    (bits, one)
                                }
                                None => {
                                    let zero_bits = self.builder.ins().iconst(types::I64, 1);
                                    let zero = self.builder.ins().iconst(types::I64, 0);
                                    (zero_bits, zero)
                                }
                            };
                            let slice_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_slice,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(slice_ref, &[subject, offset, bits, has_bits]);
                            let result = self.builder.inst_results(call)[0];
                            self.dec(offset);
                            result
                        }
                    };
                    let variable = self.builder.declare_var(types::I64);
                    self.builder.def_var(variable, value);
                    let _ = self.environment.insert(name.clone(), variable);
                    binding_slots.push((name.clone(), variable));
                }
                let result = match mode {
                    DecisionMode::Case { tail } => {
                        let scope_start = self.scope_owned.len();
                        let tail = tail.map(|mut cleanups| {
                            for (_, slot) in &binding_slots {
                                cleanups.push(self.builder.use_var(*slot));
                            }
                            cleanups
                        });
                        match self.statements_scoped(body, scope_start, tail)? {
                            None => {
                                // The clause tail-transferred; nothing joins.
                                if let Some(environment) = saved_environment {
                                    self.environment = environment;
                                }
                                return Ok(false);
                            }
                            Some(result) => {
                                // Clause bindings end with the clause.
                                for (_, slot) in binding_slots {
                                    let value = self.builder.use_var(slot);
                                    self.dec(value);
                                }
                                result
                            }
                        }
                    }
                    DecisionMode::Assignment { result, .. } => {
                        // Assignment bindings live on in the enclosing scope.
                        self.scope_owned.extend(binding_slots);
                        result
                    }
                };
                self.builder.ins().jump(join, &[result.into()]);
                if let Some(environment) = saved_environment {
                    self.environment = environment;
                }
                Ok(true)
            }

            native_ir::Decision::Fail => match mode {
                // A failed `let assert` panics with its source location.
                DecisionMode::Assignment {
                    on_failure: Some(failure),
                    ..
                } => {
                    let result = self.emit_panic(
                        2,
                        failure.message.as_deref(),
                        &failure.function.clone(),
                        failure.line,
                    )?;
                    self.builder.ins().jump(join, &[result.into()]);
                    Ok(true)
                }
                // Otherwise the type system guarantees exhaustiveness; this
                // node is unreachable at run time.
                _ => {
                    self.builder
                        .ins()
                        .trap(TrapCode::user(1).expect("valid trap code"));
                    Ok(false)
                }
            },

            native_ir::Decision::Guard {
                bindings,
                guard,
                if_true,
                if_false,
            } => {
                // The pattern's bindings are in scope for the guard and the
                // clause body, but not for the rest of the tree.
                let saved_environment = self.environment.clone();
                let mut binding_slots = Vec::with_capacity(bindings.len());
                for (name, bound) in bindings {
                    let value = match bound {
                        native_ir::Bound::Variable(id) => {
                            let value = *variables
                                .get(id)
                                .ok_or_else(|| format!("unbound decision variable {id}"))?;
                            self.inc(value)
                        }
                        native_ir::Bound::Value(expression) => self.expression(expression)?,
                        native_ir::Bound::StringSlice { subject, offset } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let offset = self.builder.ins().iconst(types::I64, *offset as i64);
                            let slice_ref = self.module.declare_func_in_func(
                                self.runtime.string_slice_from,
                                self.builder.func,
                            );
                            let call = self.builder.ins().call(slice_ref, &[subject, offset]);
                            self.builder.inst_results(call)[0]
                        }
                        native_ir::Bound::BitsReadInt {
                            subject,
                            offset,
                            bits,
                            endian,
                            signed,
                        } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let read = native_ir::SegmentRead {
                                name: String::new(),
                                offset: offset.clone(),
                                bits: bits.clone(),
                                endian: *endian,
                                signed: *signed,
                            };
                            self.emit_bits_read(subject, &read)?
                        }
                        native_ir::Bound::BitsReadFloat {
                            subject,
                            offset,
                            bits,
                            endian,
                        } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let offset = self.expression(offset)?;
                            let bits = self.expression(bits)?;
                            let endian = self.endian_flag(*endian);
                            let read_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_read_float,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(read_ref, &[subject, offset, bits, endian]);
                            let result = self.builder.inst_results(call)[0];
                            self.dec(offset);
                            self.dec(bits);
                            result
                        }
                        native_ir::Bound::BitsSlice {
                            subject,
                            offset,
                            bits,
                        } => {
                            let subject = *variables
                                .get(subject)
                                .ok_or_else(|| format!("unbound decision variable {subject}"))?;
                            let offset = self.expression(offset)?;
                            let (bits, has_bits) = match bits {
                                Some(bits) => {
                                    let bits = self.expression(bits)?;
                                    let one = self.builder.ins().iconst(types::I64, 1);
                                    (bits, one)
                                }
                                None => {
                                    let zero_bits = self.builder.ins().iconst(types::I64, 1);
                                    let zero = self.builder.ins().iconst(types::I64, 0);
                                    (zero_bits, zero)
                                }
                            };
                            let slice_ref = self.module.declare_func_in_func(
                                self.runtime.bitarray_slice,
                                self.builder.func,
                            );
                            let call = self
                                .builder
                                .ins()
                                .call(slice_ref, &[subject, offset, bits, has_bits]);
                            let result = self.builder.inst_results(call)[0];
                            self.dec(offset);
                            result
                        }
                    };
                    let variable = self.builder.declare_var(types::I64);
                    self.builder.def_var(variable, value);
                    let _ = self.environment.insert(name.clone(), variable);
                    binding_slots.push(variable);
                }

                let guard_value = self.expression(guard)?;
                let is_true = self.builder.ins().band_imm_u(guard_value, 2);
                let true_block = self.builder.create_block();
                let false_block = self.builder.create_block();
                self.builder
                    .ins()
                    .brif(is_true, true_block, &[], false_block, &[]);
                self.builder.seal_block(true_block);
                self.builder.seal_block(false_block);

                self.builder.switch_to_block(true_block);
                let jumped = match &mode {
                    DecisionMode::Case { tail: Some(outer) } => {
                        let mut cleanups = outer.clone();
                        for slot in &binding_slots {
                            cleanups.push(self.builder.use_var(*slot));
                        }
                        let scope_start = self.scope_owned.len();
                        match self.statements_scoped(if_true, scope_start, Some(cleanups))? {
                            None => false,
                            Some(result) => {
                                for slot in &binding_slots {
                                    let value = self.builder.use_var(*slot);
                                    self.dec(value);
                                }
                                self.builder.ins().jump(join, &[result.into()]);
                                true
                            }
                        }
                    }
                    _ => {
                        let result = self.statements(if_true)?;
                        for slot in &binding_slots {
                            let value = self.builder.use_var(*slot);
                            self.dec(value);
                        }
                        self.builder.ins().jump(join, &[result.into()]);
                        true
                    }
                };
                self.environment = saved_environment;

                self.builder.switch_to_block(false_block);
                for slot in &binding_slots {
                    let value = self.builder.use_var(*slot);
                    self.dec(value);
                }
                Ok(self.decision(variables, if_false, join, mode)? || jumped)
            }

            native_ir::Decision::Switch {
                var,
                choices,
                fallback,
                fallback_fields,
            } => {
                let subject = *variables
                    .get(var)
                    .ok_or_else(|| format!("unbound decision variable {var}"))?;
                let mut joined = false;
                for (check, decision) in choices {
                    let matched = match check {
                        native_ir::Check::Variant { tag, fields } => {
                            let actual = self.builder.ins().load(
                                types::I64,
                                MemFlagsData::trusted(),
                                subject,
                                0,
                            );
                            // The display id in the header's top bits is not
                            // semantic.
                            let actual = self.builder.ins().band_imm_u(
                                actual,
                                native_runtime::HEADER_SEMANTIC_MASK as i64,
                            );
                            let expected = self.builder.ins().iconst(
                                types::I64,
                                native_runtime::record_header(*tag, fields.len() as u32) as i64,
                            );
                            self.builder.ins().icmp(IntCC::Equal, actual, expected)
                        }
                        // Tuples always match: the type system guarantees it.
                        native_ir::Check::Always { .. } => {
                            self.builder.ins().iconst(types::I64, 1)
                        }
                        // Any list value that is not the empty immediate is
                        // a cons cell.
                        native_ir::Check::NonEmptyList { .. } => {
                            self.builder.ins().icmp_imm_s(IntCC::NotEqual, subject, NIL)
                        }
                        check => self.check(subject, check)?,
                    };
                    let match_block = self.builder.create_block();
                    let next_block = self.builder.create_block();
                    self.builder
                        .ins()
                        .brif(matched, match_block, &[], next_block, &[]);
                    self.builder.seal_block(match_block);
                    self.builder.seal_block(next_block);

                    self.builder.switch_to_block(match_block);
                    // A matched variant, tuple, or cons cell makes its
                    // fields available as decision variables for the rest of
                    // this branch. All three share the record layout: fields
                    // start one word past the tag.
                    let extracted: Vec<u32> = match check {
                        native_ir::Check::Variant { fields, .. }
                        | native_ir::Check::Always { fields } => fields.clone(),
                        native_ir::Check::NonEmptyList { first, rest } => vec![*first, *rest],
                        _ => vec![],
                    };
                    if extracted.is_empty() {
                        joined |= self.decision(variables, decision, join, mode.clone())?;
                    } else {
                        let mut extended = variables.clone();
                        for (index, field) in extracted.iter().enumerate() {
                            let value = self.builder.ins().load(
                                types::I64,
                                MemFlagsData::trusted(),
                                subject,
                                8 + 8 * index as i32,
                            );
                            let _ = extended.insert(*field, value);
                        }
                        joined |= self.decision(&extended, decision, join, mode.clone())?;
                    }
                    self.builder.switch_to_block(next_block);
                }
                // An exhaustive match's final variant is not tag-tested,
                // but its fields still become decision variables.
                if fallback_fields.is_empty() {
                    Ok(self.decision(variables, fallback, join, mode)? || joined)
                } else {
                    let mut extended = variables.clone();
                    for (index, field) in fallback_fields.iter().enumerate() {
                        let value = self.builder.ins().load(
                            types::I64,
                            MemFlagsData::trusted(),
                            subject,
                            8 + 8 * index as i32,
                        );
                        let _ = extended.insert(*field, value);
                    }
                    Ok(self.decision(&extended, fallback, join, mode)? || joined)
                }
            }
        }
    }

    /// Emits a runtime check against a subject, yielding a value that is
    /// non-zero when the check succeeds.
    fn check(&mut self, subject: Value, check: &native_ir::Check) -> Result<Value, String> {
        match check {
            native_ir::Check::Int(value) => {
                let expected = self.builder.ins().iconst(types::I64, (value << 1) | 1);
                Ok(self.builder.ins().icmp(IntCC::Equal, subject, expected))
            }
            native_ir::Check::Immediate(word) => {
                let expected = self.builder.ins().iconst(types::I64, *word);
                Ok(self.builder.ins().icmp(IntCC::Equal, subject, expected))
            }
            native_ir::Check::BigInt(bytes) => {
                let literal =
                    self.construct_from_constant_bytes(bytes, self.runtime.bigint_from_bytes)?;
                let equal = self.int_compare(IntCC::Equal, subject, literal)?;
                self.dec(literal);
                Ok(self.builder.ins().band_imm_u(equal, 2))
            }
            native_ir::Check::Float(value) => {
                let subject = self.load_float(subject);
                let expected = self.builder.ins().f64const(*value);
                Ok(self.builder.ins().fcmp(FloatCC::Equal, subject, expected))
            }
            native_ir::Check::Variant { .. }
            | native_ir::Check::Always { .. }
            | native_ir::Check::NonEmptyList { .. } => {
                unreachable!("field-extracting checks are handled by the switch")
            }

            native_ir::Check::StringPrefix { prefix } => {
                let (pointer, length) = self.constant_bytes(prefix.as_bytes())?;
                let starts_with_ref = self
                    .module
                    .declare_func_in_func(self.runtime.string_starts_with, self.builder.func);
                let call = self
                    .builder
                    .ins()
                    .call(starts_with_ref, &[subject, pointer, length]);
                let matched = self.builder.inst_results(call)[0];
                Ok(self.builder.ins().band_imm_u(matched, 2))
            }
            native_ir::Check::BitArray { reads, test } => {
                // Materialize the segment reads the test refers to; the
                // values stay bound for later checks and bindings on this
                // path.
                for read in reads {
                    let value = self.emit_bits_read(subject, read)?;
                    let variable = self.builder.declare_var(types::I64);
                    self.builder.def_var(variable, value);
                    let _ = self.environment.insert(read.name.clone(), variable);
                }
                match test {
                    native_ir::BitsTest::AlwaysTrue => {
                        Ok(self.builder.ins().iconst(types::I64, 1))
                    }
                    native_ir::BitsTest::NonNegative { value } => {
                        let value = self.expression(value)?;
                        let zero = self.builder.ins().iconst(types::I64, 1);
                        let result =
                            self.int_compare(IntCC::SignedGreaterThanOrEqual, value, zero)?;
                        self.dec(value);
                        Ok(self.builder.ins().band_imm_u(result, 2))
                    }
                    native_ir::BitsTest::Size { bits, exact } => {
                        let bits = self.expression(bits)?;
                        let exact = self.builder.ins().iconst(types::I64, *exact as i64);
                        let test_ref = self.module.declare_func_in_func(
                            self.runtime.bitarray_size_test,
                            self.builder.func,
                        );
                        let call = self.builder.ins().call(test_ref, &[subject, bits, exact]);
                        let matched = self.builder.inst_results(call)[0];
                        self.dec(bits);
                        Ok(self.builder.ins().band_imm_u(matched, 2))
                    }
                    native_ir::BitsTest::Bytes {
                        offset,
                        bytes,
                        bit_length,
                    } => {
                        let offset = self.expression(offset)?;
                        let (pointer, _byte_length) = self.constant_bytes(bytes)?;
                        let bit_length =
                            self.builder.ins().iconst(types::I64, *bit_length as i64);
                        let test_ref = self.module.declare_func_in_func(
                            self.runtime.bitarray_bytes_test,
                            self.builder.func,
                        );
                        let call = self
                            .builder
                            .ins()
                            .call(test_ref, &[subject, offset, pointer, bit_length]);
                        let matched = self.builder.inst_results(call)[0];
                        self.dec(offset);
                        Ok(self.builder.ins().band_imm_u(matched, 2))
                    }
                    native_ir::BitsTest::IsFiniteFloat {
                        offset,
                        bits,
                        endian,
                    } => {
                        let offset = self.expression(offset)?;
                        let bits = self.expression(bits)?;
                        let endian = self.endian_flag(*endian);
                        let test_ref = self.module.declare_func_in_func(
                            self.runtime.bitarray_is_finite_float,
                            self.builder.func,
                        );
                        let call = self
                            .builder
                            .ins()
                            .call(test_ref, &[subject, offset, bits, endian]);
                        let matched = self.builder.inst_results(call)[0];
                        self.dec(offset);
                        self.dec(bits);
                        Ok(self.builder.ins().band_imm_u(matched, 2))
                    }
                    native_ir::BitsTest::RestIsBytes { offset } => {
                        let offset = self.expression(offset)?;
                        let test_ref = self.module.declare_func_in_func(
                            self.runtime.bitarray_rest_is_bytes,
                            self.builder.func,
                        );
                        let call = self.builder.ins().call(test_ref, &[subject, offset]);
                        let matched = self.builder.inst_results(call)[0];
                        self.dec(offset);
                        Ok(self.builder.ins().band_imm_u(matched, 2))
                    }
                }
            }
            native_ir::Check::String(value) => {
                let literal = self
                    .construct_from_constant_bytes(value.as_bytes(), self.runtime.string_from_bytes)?;
                let eq_ref = self
                    .module
                    .declare_func_in_func(self.runtime.string_eq, self.builder.func);
                let call = self.builder.ins().call(eq_ref, &[subject, literal]);
                let equal = self.builder.inst_results(call)[0];
                self.dec(literal);
                Ok(self.builder.ins().band_imm_u(equal, 2))
            }
        }
    }

    /// Loads the f64 out of a boxed float value, past its header word.
    fn load_float(&mut self, boxed: Value) -> Value {
        self.builder
            .ins()
            .load(types::F64, MemFlagsData::trusted(), boxed, 8)
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
        condition: IntCC,
        left: Value,
        right: Value,
    ) -> Result<Value, String> {
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

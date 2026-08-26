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

use std::collections::{HashMap, HashSet};

use cranelift_codegen::ir::condcodes::{FloatCC, IntCC};
use cranelift_codegen::ir::{
    AbiParam, Block, InstBuilder, MemFlagsData, Signature, TrapCode, Value, types,
};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module};

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

/// The symbol of the runtime's destructor for an object whose reference
/// count has reached zero: the slow path behind the decrement generated
/// code emits inline.
pub const DESTROY: &str = "gleam_native_destroy";

/// The symbol of the runtime data word holding the program thread's
/// allocation pool address, read by inline pooled allocations.
pub const POOL: &str = "gleam_native_pool";

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
    destroy: FuncId,
    panic: FuncId,
    /// The pool-address data symbol, for inline pooled allocation.
    pool: DataId,
}

impl RuntimeFunctions {
    fn declare(module: &mut impl Module) -> Result<Self, String> {
        let call_conv = module.isa().default_call_conv();
        let pool = module
            .declare_data(POOL, Linkage::Import, true, false)
            .map_err(|error| error.to_string())?;
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
            destroy: declare(DESTROY, 1)?,
            panic: declare(PANIC, 7)?,
            pool,
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
            dying: HashSet::new(),
            consumed: Vec::new(),
            reuse_token: None,
            branch_depth: 0,
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
        let target_ref = self.module.declare_func_in_func(target, builder.func);
        if target_external {
            // Externals borrow: call, then release what this wrapper owns.
            let call = builder.ins().call(target_ref, &arguments);
            let result = builder.inst_results(call)[0];
            emit_dec(self.module, self.runtime.destroy, &mut builder, closure);
            for argument in &arguments {
                emit_dec(self.module, self.runtime.destroy, &mut builder, *argument);
            }
            builder.ins().return_(&[result]);
        } else {
            // Gleam functions own their arguments: release the closure and
            // transfer the rest with a genuine tail call.
            emit_dec(self.module, self.runtime.destroy, &mut builder, closure);
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
            dying: HashSet::new(),
            consumed: Vec::new(),
            reuse_token: None,
            branch_depth: 0,
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

/// Whether an expression evaluates every one of its variable mentions
/// exactly once whenever it is evaluated: no branches (`case`,
/// short-circuiting booleans), no nested scopes (blocks), and no deferred
/// bodies (lambdas, whose mentions become captures evaluated elsewhere).
/// Only such expressions may consume a binding at its last use.
fn straight_line(expression: &native_ir::Expression) -> bool {
    use native_ir::Expression;
    match expression {
        Expression::Case { .. }
        | Expression::BoolBinary { .. }
        | Expression::Lambda { .. }
        | Expression::Block(_) => false,
        Expression::Int(_)
        | Expression::BigInt(_)
        | Expression::Float(_)
        | Expression::String(_)
        | Expression::Nil
        | Expression::Bool(_)
        | Expression::Variable(_)
        | Expression::EmptyList
        | Expression::FunctionReference { .. } => true,
        Expression::Call { arguments, .. } | Expression::Constructor { arguments, .. } => {
            arguments.iter().all(straight_line)
        }
        Expression::CallValue { callee, arguments } => {
            straight_line(callee) && arguments.iter().all(straight_line)
        }
        Expression::IntBinary { left, right, .. }
        | Expression::IntCompare { left, right, .. }
        | Expression::FloatBinary { left, right, .. }
        | Expression::FloatCompare { left, right, .. }
        | Expression::Equality { left, right, .. }
        | Expression::StringConcat(left, right) => {
            straight_line(left) && straight_line(right)
        }
        Expression::BoolNot(inner) => straight_line(inner),
        Expression::FieldAccess { record, .. } => straight_line(record),
        Expression::BitArray(segments) => segments.iter().all(|segment| {
            straight_line(&segment.value)
                && match &segment.kind {
                    native_ir::BitSegmentKind::Int { bits, .. }
                    | native_ir::BitSegmentKind::Float { bits, .. } => straight_line(bits),
                    native_ir::BitSegmentKind::BitArraySplice { bits: Some(bits) } => {
                        straight_line(bits)
                    }
                    native_ir::BitSegmentKind::BitArraySplice { bits: None }
                    | native_ir::BitSegmentKind::String { .. }
                    | native_ir::BitSegmentKind::Codepoint { .. } => true,
                }
        }),
        // A panic aborts: mentions before it still evaluate exactly once.
        Expression::Echo { value, message, .. } => {
            straight_line(value) && message.as_deref().is_none_or(straight_line)
        }
        Expression::Panic { message, .. } => message.as_deref().is_none_or(straight_line),
    }
}

/// Scans a clause body in evaluation order for the first record
/// construction of `arity` fields. `Some(true)` means it is evaluated
/// unconditionally whenever the body runs, so a claimed reuse token is
/// guaranteed to be consumed; `Some(false)` means it sits behind a branch;
/// `None` means the body never constructs that shape. Lambda bodies are
/// skipped: they translate as separate functions and cannot consume this
/// clause's token. Nested decision trees are conservatively treated as
/// containing a conditional site.
fn first_reuse_site(
    statements: &[native_ir::Statement],
    arity: usize,
    conditional: bool,
) -> Option<bool> {
    for statement in statements {
        let found = match statement {
            native_ir::Statement::Let { value, .. }
            | native_ir::Statement::Expression(value) => reuse_site_in(value, arity, conditional),
            native_ir::Statement::Destructure { subject, .. } => reuse_site_in(subject, arity, conditional)
                // The tree and failure message may construct behind checks.
                .or(Some(false)),
        };
        if found.is_some() {
            return found;
        }
    }
    None
}

/// [`first_reuse_site`] for one expression, walking in evaluation order.
fn reuse_site_in(
    expression: &native_ir::Expression,
    arity: usize,
    conditional: bool,
) -> Option<bool> {
    use native_ir::Expression;
    match expression {
        Expression::Constructor { arguments, .. } => {
            for argument in arguments {
                if let Some(found) = reuse_site_in(argument, arity, conditional) {
                    return Some(found);
                }
            }
            if arguments.len() == arity {
                Some(!conditional)
            } else {
                None
            }
        }
        Expression::Case { subjects, .. } => {
            for subject in subjects {
                if let Some(found) = reuse_site_in(subject, arity, conditional) {
                    return Some(found);
                }
            }
            // Clause bodies run behind checks; treat the tree as holding a
            // conditional site rather than walking it.
            Some(false)
        }
        Expression::BoolBinary { left, right, .. } => {
            if let Some(found) = reuse_site_in(left, arity, conditional) {
                return Some(found);
            }
            // The right side only runs when the left does not decide.
            reuse_site_in(right, arity, true)
        }
        Expression::Lambda { .. } => None,
        Expression::Block(statements) => first_reuse_site(statements, arity, conditional),
        Expression::Call { arguments, .. } => {
            for argument in arguments {
                if let Some(found) = reuse_site_in(argument, arity, conditional) {
                    return Some(found);
                }
            }
            None
        }
        Expression::CallValue { callee, arguments } => {
            if let Some(found) = reuse_site_in(callee, arity, conditional) {
                return Some(found);
            }
            for argument in arguments {
                if let Some(found) = reuse_site_in(argument, arity, conditional) {
                    return Some(found);
                }
            }
            None
        }
        Expression::IntBinary { left, right, .. }
        | Expression::IntCompare { left, right, .. }
        | Expression::FloatBinary { left, right, .. }
        | Expression::FloatCompare { left, right, .. }
        | Expression::Equality { left, right, .. }
        | Expression::StringConcat(left, right) => {
            if let Some(found) = reuse_site_in(left, arity, conditional) {
                return Some(found);
            }
            reuse_site_in(right, arity, conditional)
        }
        Expression::BoolNot(inner) => reuse_site_in(inner, arity, conditional),
        Expression::FieldAccess { record, .. } => reuse_site_in(record, arity, conditional),
        Expression::BitArray(segments) => {
            for segment in segments {
                if let Some(found) = reuse_site_in(&segment.value, arity, conditional) {
                    return Some(found);
                }
                let bits = match &segment.kind {
                    native_ir::BitSegmentKind::Int { bits, .. }
                    | native_ir::BitSegmentKind::Float { bits, .. } => Some(bits),
                    native_ir::BitSegmentKind::BitArraySplice { bits } => bits.as_ref(),
                    native_ir::BitSegmentKind::String { .. }
                    | native_ir::BitSegmentKind::Codepoint { .. } => None,
                };
                if let Some(bits) = bits
                    && let Some(found) = reuse_site_in(bits, arity, conditional)
                {
                    return Some(found);
                }
            }
            None
        }
        Expression::Echo { value, message, .. } => {
            if let Some(found) = reuse_site_in(value, arity, conditional) {
                return Some(found);
            }
            message
                .as_deref()
                .and_then(|message| reuse_site_in(message, arity, conditional))
        }
        Expression::Panic { message, .. } => message
            .as_deref()
            .and_then(|message| reuse_site_in(message, arity, conditional)),
        Expression::Int(_)
        | Expression::BigInt(_)
        | Expression::Float(_)
        | Expression::String(_)
        | Expression::Nil
        | Expression::Bool(_)
        | Expression::Variable(_)
        | Expression::EmptyList
        | Expression::FunctionReference { .. } => None,
    }
}

/// Emits an inline reference count increment: nothing for immediates, one
/// added to the count word before the object for heap values.
fn emit_inc(builder: &mut FunctionBuilder<'_>, value: Value) {
    let heap = builder.create_block();
    let done = builder.create_block();
    let immediate = builder.ins().band_imm_u(value, 1);
    builder.ins().brif(immediate, done, &[], heap, &[]);
    builder.seal_block(heap);

    builder.switch_to_block(heap);
    let count = builder
        .ins()
        .load(types::I64, MemFlagsData::trusted(), value, -8);
    let incremented = builder.ins().iadd_imm_s(count, 1);
    let _ = builder
        .ins()
        .store(MemFlagsData::trusted(), incremented, value, -8);
    builder.ins().jump(done, &[]);
    builder.seal_block(done);

    builder.switch_to_block(done);
}

/// Emits an inline reference count decrement: nothing for immediates, one
/// subtracted from the count word for heap values, and a cold runtime
/// destroy call when the count reaches zero.
fn emit_dec<M: Module>(
    module: &mut M,
    destroy: FuncId,
    builder: &mut FunctionBuilder<'_>,
    value: Value,
) {
    let heap = builder.create_block();
    let dead = builder.create_block();
    let done = builder.create_block();
    let immediate = builder.ins().band_imm_u(value, 1);
    builder.ins().brif(immediate, done, &[], heap, &[]);
    builder.seal_block(heap);

    builder.switch_to_block(heap);
    let count = builder
        .ins()
        .load(types::I64, MemFlagsData::trusted(), value, -8);
    let decremented = builder.ins().iadd_imm_s(count, -1);
    let _ = builder
        .ins()
        .store(MemFlagsData::trusted(), decremented, value, -8);
    builder.ins().brif(decremented, done, &[], dead, &[]);
    builder.seal_block(dead);
    builder.set_cold_block(dead);

    builder.switch_to_block(dead);
    let destroy_ref = module.declare_func_in_func(destroy, builder.func);
    let _ = builder.ins().call(destroy_ref, &[value]);
    builder.ins().jump(done, &[]);
    builder.seal_block(done);

    builder.switch_to_block(done);
}

/// The owned values a tail transfer must release between argument
/// evaluation and the transfer: enclosing scope bindings (with their names,
/// so a last-use argument can consume its entry) and unnamed temporaries
/// such as case subjects.
type Cleanups = Vec<(Option<String>, Value)>;

/// How a decision tree's leaves behave: `case` clause bodies run in their
/// own scope, while assignment destructuring binds into the enclosing scope
/// and yields the subject value.
#[derive(Clone)]
enum DecisionMode<'a> {
    Case {
        /// In tail position: the owned values (enclosing scopes, subjects)
        /// to release before a tail transfer inside a clause.
        tail: Option<Cleanups>,
        /// The case's subject values: the only values a clause may claim
        /// for drop-reuse (nested switches scrutinize borrowed fields).
        subjects: Vec<Value>,
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
    /// their scope's statement sequence finishes — or earlier, as soon as
    /// no remaining statement mentions them, so that tail-recursive loops
    /// release their garbage every iteration.
    scope_owned: Vec<(String, Variable)>,
    /// Bindings whose next use is their last: that use consumes the
    /// binding's ownership instead of incrementing here and releasing at
    /// the scope's end. Armed per straight-line statement (and per tail
    /// transfer's argument evaluation), empty otherwise.
    dying: HashSet<String>,
    /// The names consumed while [`Self::dying`] was armed, so a pending
    /// tail-transfer release list can skip their entries.
    consumed: Vec<String>,
    /// A claimed drop-reuse token: a variable holding either the memory of
    /// a uniquely-owned, already-deconstructed record of the given field
    /// count, or zero. The first constructor of that field count writes
    /// into it instead of allocating.
    reuse_token: Option<(Variable, usize)>,
    /// How many decision-tree clause bodies enclose the current
    /// translation point. Subject-position consumption is sound only at
    /// depth zero: inside a clause, sibling paths that did not run the
    /// subject evaluation would need a different release state at the
    /// join.
    branch_depth: usize,
}

/// A subject a clause may deconstruct in place: the matched record value
/// and the decision variables its fields were extracted into.
struct ReuseCandidate {
    subject: Value,
    fields: Vec<u32>,
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
        let header = native_runtime::closure_header(captures.len() as u32, arity);
        let closure = self.pool_alloc(2 + captures.len(), header, |this| {
            let capture_count = this.builder.ins().iconst(types::I64, captures.len() as i64);
            let arity = this.builder.ins().iconst(types::I64, arity as i64);
            let closure_new_ref = this
                .module
                .declare_func_in_func(this.runtime.closure_new, this.builder.func);
            let call = this
                .builder
                .ins()
                .call(closure_new_ref, &[capture_count, arity]);
            Ok(this.builder.inst_results(call)[0])
        })?;

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
        emit_inc(self.builder, value);
        value
    }

    /// Emits a reference count decrement.
    fn dec(&mut self, value: Value) {
        emit_dec(self.module, self.runtime.destroy, self.builder, value);
    }

    /// Emits a per-site permanent cache: a writable data slot holds a
    /// value built at most once (cold path), and every evaluation takes
    /// its own reference. The slot's own reference is never released, so
    /// only immutable values whose sharing is unobservable belong here —
    /// string literals and zero-arity constructors.
    fn interned(
        &mut self,
        build: impl FnOnce(&mut Self) -> Result<Value, String>,
    ) -> Result<Value, String> {
        let slot = self
            .module
            .declare_anonymous_data(true, false)
            .map_err(|error| error.to_string())?;
        let mut description = DataDescription::new();
        description.define_zeroinit(8);
        description.set_align(8);
        self.module
            .define_data(slot, &description)
            .map_err(|error| error.to_string())?;
        let slot_ref = self.module.declare_data_in_func(slot, self.builder.func);
        let pointer_type = self.module.target_config().pointer_type();
        let slot_address = self.builder.ins().symbol_value(pointer_type, slot_ref);
        let cached = self.builder.ins().load(
            types::I64,
            MemFlagsData::trusted(),
            slot_address,
            0,
        );

        let build_block = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.append_block_param(join, types::I64);
        self.builder
            .ins()
            .brif(cached, join, &[cached.into()], build_block, &[]);
        self.builder.seal_block(build_block);
        self.builder.set_cold_block(build_block);

        self.builder.switch_to_block(build_block);
        let built = build(self)?;
        let _ = self
            .builder
            .ins()
            .store(MemFlagsData::trusted(), built, slot_address, 0);
        self.builder.ins().jump(join, &[built.into()]);
        self.builder.seal_block(join);

        self.builder.switch_to_block(join);
        let value = self.builder.block_params(join)[0];
        Ok(self.inc(value))
    }

    /// Emits an inline pooled allocation of `object_words` words (header
    /// included) with the given constant header word, popping the program
    /// thread's free list for the size class. `slow` emits the fallback
    /// runtime call, taken when the pool is empty or not yet published —
    /// or, for objects too large to pool, unconditionally. The fallback
    /// also writes the header, so only field words remain to store.
    fn pool_alloc(
        &mut self,
        object_words: usize,
        header: u64,
        slow: impl FnOnce(&mut Self) -> Result<Value, String>,
    ) -> Result<Value, String> {
        // The class index is the block's total size, count word included,
        // mirroring the runtime's pool layout.
        let total = 1 + object_words;
        if total >= native_runtime::POOL_CLASSES {
            return slow(self);
        }
        let check = self.builder.create_block();
        let hit = self.builder.create_block();
        let miss = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.append_block_param(join, types::I64);

        let pool_ref = self
            .module
            .declare_data_in_func(self.runtime.pool, self.builder.func);
        let pointer_type = self.module.target_config().pointer_type();
        let pool_address = self.builder.ins().symbol_value(pointer_type, pool_ref);
        let pool =
            self.builder
                .ins()
                .load(types::I64, MemFlagsData::trusted(), pool_address, 0);
        self.builder.ins().brif(pool, check, &[], miss, &[]);
        self.builder.seal_block(check);

        self.builder.switch_to_block(check);
        let head_offset = (8 * total) as i32;
        let head = self
            .builder
            .ins()
            .load(types::I64, MemFlagsData::trusted(), pool, head_offset);
        self.builder.ins().brif(head, hit, &[], miss, &[]);
        self.builder.seal_block(hit);
        self.builder.seal_block(miss);
        self.builder.set_cold_block(miss);

        // Pop the block: its count word holds the next free block. The
        // count becomes one (this allocation's reference) and the header
        // is a compile-time constant.
        self.builder.switch_to_block(hit);
        let next = self
            .builder
            .ins()
            .load(types::I64, MemFlagsData::trusted(), head, 0);
        let _ = self
            .builder
            .ins()
            .store(MemFlagsData::trusted(), next, pool, head_offset);
        let blocks_offset = (8 * (native_runtime::POOL_CLASSES + total)) as i32;
        let blocks =
            self.builder
                .ins()
                .load(types::I64, MemFlagsData::trusted(), pool, blocks_offset);
        let blocks = self.builder.ins().iadd_imm_s(blocks, -1);
        let _ = self
            .builder
            .ins()
            .store(MemFlagsData::trusted(), blocks, pool, blocks_offset);
        let one = self.builder.ins().iconst(types::I64, 1);
        let _ = self.builder.ins().store(MemFlagsData::trusted(), one, head, 0);
        let header_value = self.builder.ins().iconst(types::I64, header as i64);
        let _ = self
            .builder
            .ins()
            .store(MemFlagsData::trusted(), header_value, head, 8);
        let value = self.builder.ins().iadd_imm_s(head, 8);
        self.builder.ins().jump(join, &[value.into()]);

        self.builder.switch_to_block(miss);
        let slow_value = slow(self)?;
        self.builder.ins().jump(join, &[slow_value.into()]);
        self.builder.seal_block(join);

        self.builder.switch_to_block(join);
        Ok(self.builder.block_params(join)[0])
    }

    /// Arms last-use consumption for a tail transfer's argument evaluation:
    /// a name owed a release in `cleanups` whose single mention across the
    /// argument expressions is about to be evaluated unconditionally is
    /// consumed at that use instead of incremented there and released
    /// before the transfer.
    fn arm_last_uses(
        &mut self,
        mentioned: &[&native_ir::Expression],
        cleanups: &[(Option<String>, Value)],
    ) {
        self.dying.clear();
        self.consumed.clear();
        if !mentioned.iter().all(|expression| straight_line(expression)) {
            return;
        }
        let mut counts = HashMap::new();
        for expression in mentioned {
            native_ir::expression_mentions(expression, &mut counts);
        }
        for (name, _) in cleanups {
            if let Some(name) = name
                && counts.get(name) == Some(&1)
            {
                let _ = self.dying.insert(name.clone());
            }
        }
    }

    /// Removes the entries whose names were consumed while [`Self::dying`]
    /// was armed (innermost entry per consumed name) and disarms.
    fn remove_consumed(&mut self, cleanups: &mut Cleanups) {
        self.dying.clear();
        for name in std::mem::take(&mut self.consumed) {
            if let Some(position) = cleanups
                .iter()
                .rposition(|(entry, _)| entry.as_deref() == Some(name.as_str()))
            {
                let _ = cleanups.remove(position);
            }
        }
    }

    /// Releases a tail transfer's owed values, minus the entries consumed
    /// during argument evaluation.
    fn release_cleanups(&mut self, mut cleanups: Cleanups) {
        self.remove_consumed(&mut cleanups);
        for (_, value) in cleanups {
            self.dec(value);
        }
    }

    /// Whether a clause body in tail position is certain to end in a tail
    /// transfer, so its join path is unreachable.
    fn body_transfers(&self, body: &[native_ir::Statement]) -> bool {
        match body.last() {
            Some(native_ir::Statement::Expression(native_ir::Expression::Call {
                module,
                function,
                ..
            })) => self
                .functions
                .get(&(module.clone(), function.clone()))
                .is_some_and(|(_, external)| !external),
            Some(native_ir::Statement::Expression(native_ir::Expression::CallValue {
                ..
            })) => true,
            _ => false,
        }
    }

    /// Emits the release of a matched subject at clause entry. When its
    /// count is one this claims the block as a drop-reuse token, releasing
    /// the fields it owned (pattern bindings hold their own references);
    /// otherwise the count just drops, other owners keeping the record
    /// alive. Returns the token variable: the block's value pointer, or
    /// zero when the record was shared.
    fn emit_reuse_claim(
        &mut self,
        candidate: &ReuseCandidate,
        variables: &HashMap<u32, Value>,
    ) -> Result<Variable, String> {
        let token = self.builder.declare_var(types::I64);
        let claim = self.builder.create_block();
        let shared = self.builder.create_block();
        let done = self.builder.create_block();
        let count = self.builder.ins().load(
            types::I64,
            MemFlagsData::trusted(),
            candidate.subject,
            -8,
        );
        let unique = self.builder.ins().icmp_imm_s(IntCC::Equal, count, 1);
        self.builder.ins().brif(unique, claim, &[], shared, &[]);
        self.builder.seal_block(claim);
        self.builder.seal_block(shared);

        self.builder.switch_to_block(claim);
        for field in &candidate.fields {
            let value = *variables
                .get(field)
                .ok_or_else(|| format!("unbound decision variable {field}"))?;
            self.dec(value);
        }
        self.builder.def_var(token, candidate.subject);
        self.builder.ins().jump(done, &[]);

        self.builder.switch_to_block(shared);
        let decremented = self.builder.ins().iadd_imm_s(count, -1);
        let _ = self.builder.ins().store(
            MemFlagsData::trusted(),
            decremented,
            candidate.subject,
            -8,
        );
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(token, zero);
        self.builder.ins().jump(done, &[]);
        self.builder.seal_block(done);

        self.builder.switch_to_block(done);
        Ok(token)
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
        tail: Option<Cleanups>,
    ) -> Result<Option<Value>, String> {
        let mut result = None;
        for (index, statement) in statements.iter().enumerate() {
            let is_final = index + 1 == statements.len();
            // What this statement and the rest of the sequence mention.
            // Bindings mentioned by neither release now, so tail-recursive
            // loops release their garbage every iteration; a binding whose
            // single remaining mention is in this statement is consumed by
            // that use below.
            let counts = native_ir::mention_counts(std::slice::from_ref(statement));
            let suffix = native_ir::mention_counts(&statements[index + 1..]);
            let kept = self.scope_owned.split_off(scope_start);
            for (name, slot) in kept {
                if counts.contains_key(&name) || suffix.contains_key(&name) {
                    self.scope_owned.push((name, slot));
                } else {
                    let value = self.builder.use_var(slot);
                    self.dec(value);
                }
            }
            if is_final {
                // In tail position, a final call transfers control: hand it
                // the values still owed a release.
                if let (Some(outer), native_ir::Statement::Expression(expression)) =
                    (&tail, statement)
                {
                    let mut cleanups = outer.clone();
                    for (name, slot) in &self.scope_owned[scope_start..] {
                        cleanups.push((Some(name.clone()), self.builder.use_var(*slot)));
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
            // Arm last-use consumption: a binding of this scope whose only
            // remaining mention sits in a statement that evaluates it
            // unconditionally moves at that use instead.
            let armable = match statement {
                native_ir::Statement::Let { value, .. } => straight_line(value),
                native_ir::Statement::Expression(expression) => straight_line(expression),
                native_ir::Statement::Destructure { .. } => false,
            };
            if armable {
                for (name, _) in &self.scope_owned[scope_start..] {
                    if counts.get(name) == Some(&1) && !suffix.contains_key(name) {
                        let _ = self.dying.insert(name.clone());
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
                        None,
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
            self.dying.clear();
            self.consumed.clear();
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
        cleanups: Cleanups,
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
                let mentioned: Vec<&native_ir::Expression> = arguments.iter().collect();
                self.arm_last_uses(&mentioned, &cleanups);
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.expression(argument)?);
                }
                self.release_cleanups(cleanups);
                let function_ref = self.module.declare_func_in_func(id, self.builder.func);
                self.builder.ins().return_call(function_ref, &values);
                Ok(None)
            }

            native_ir::Expression::CallValue { callee, arguments } => {
                let mut mentioned: Vec<&native_ir::Expression> = vec![callee];
                mentioned.extend(arguments.iter());
                self.arm_last_uses(&mentioned, &cleanups);
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
                self.release_cleanups(cleanups);
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
                // Arm last-use consumption for subject evaluation: a binding
                // owed a release whose single mention in the whole case is
                // one subject moves into the subject temporary. This is what
                // keeps a scrutinized value uniquely owned, so a clause can
                // claim it for drop-reuse.
                self.dying.clear();
                self.consumed.clear();
                if self.branch_depth == 0 && subjects.iter().all(straight_line) {
                    let mut subject_counts = HashMap::new();
                    for subject in subjects {
                        native_ir::expression_mentions(subject, &mut subject_counts);
                    }
                    let mut tree_counts = HashMap::new();
                    native_ir::decision_mentions(tree, &mut tree_counts);
                    for (name, _) in &cleanups {
                        if let Some(name) = name
                            && subject_counts.get(name) == Some(&1)
                            && !tree_counts.contains_key(name)
                        {
                            let _ = self.dying.insert(name.clone());
                        }
                    }
                }
                let mut variables = HashMap::new();
                let mut subject_values = Vec::with_capacity(subjects.len());
                for (id, subject) in subject_ids.iter().zip(subjects) {
                    let value = self.expression(subject)?;
                    let _ = variables.insert(*id, value);
                    subject_values.push(value);
                }
                let mut cleanups = cleanups;
                self.remove_consumed(&mut cleanups);
                let mut tail_cleanups = cleanups;
                tail_cleanups.extend(subject_values.iter().map(|value| (None, *value)));

                let join = self.builder.create_block();
                self.builder.append_block_param(join, types::I64);
                let joined = self.decision(
                    &variables,
                    tree,
                    join,
                    DecisionMode::Case {
                        tail: Some(tail_cleanups),
                        subjects: subject_values.clone(),
                    },
                    None,
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
                // A literal builds its heap string once per site; every
                // evaluation takes its own reference. Loops re-running a
                // literal stop allocating for it.
                self.interned(|this| {
                    this.construct_from_constant_bytes(
                        string.as_bytes(),
                        this.runtime.string_from_bytes,
                    )
                })
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
                let variable = *self
                    .environment
                    .get(name)
                    .ok_or_else(|| format!("unbound variable `{name}`"))?;
                let value = self.builder.use_var(variable);
                if self.dying.remove(name) {
                    // The binding's last use: move its ownership into this
                    // use instead of an increment here and a release at the
                    // scope's end. The slot comparison guards against a
                    // name resolving to a shadowing binding (a guard
                    // binding, say) that is not the owned entry.
                    if let Some(position) = self
                        .scope_owned
                        .iter()
                        .rposition(|(owned, slot)| owned == name && *slot == variable)
                    {
                        let _ = self.scope_owned.remove(position);
                        self.consumed.push(name.clone());
                        return Ok(value);
                    }
                }
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
                let mut subject_values = Vec::with_capacity(subjects.len());
                for (id, subject) in subject_ids.iter().zip(subjects) {
                    let value = self.expression(subject)?;
                    let _ = variables.insert(*id, value);
                    subject_values.push(value);
                }

                let join = self.builder.create_block();
                self.builder.append_block_param(join, types::I64);
                let _ = self.decision(
                    &variables,
                    tree,
                    join,
                    DecisionMode::Case {
                        tail: None,
                        subjects: subject_values.clone(),
                    },
                    None,
                )?;
                self.builder.seal_block(join);

                self.builder.switch_to_block(join);
                let result = self.builder.block_params(join)[0];
                for subject in subject_values {
                    self.dec(subject);
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
                let arity = values.len();
                // The header is a compile-time constant, mirroring the
                // runtime allocator's formula.
                let header = native_runtime::record_header(*tag, arity as u32)
                    | ((display as u64) << 48);
                let slow = |this: &mut Self| {
                    let tag = this.builder.ins().iconst(types::I64, *tag as i64);
                    let arity = this.builder.ins().iconst(types::I64, arity as i64);
                    let display = this.builder.ins().iconst(types::I64, display as i64);
                    let record_new_ref = this
                        .module
                        .declare_func_in_func(this.runtime.record_new, this.builder.func);
                    let call = this
                        .builder
                        .ins()
                        .call(record_new_ref, &[tag, arity, display]);
                    Ok(this.builder.inst_results(call)[0])
                };
                if arity == 0 {
                    // A zero-arity constructor is immutable and content-equal
                    // to every other instance of its variant: one shared
                    // instance per site, like a string literal.
                    return self.interned(slow);
                }
                let reusable = match self.reuse_token {
                    Some((token, fields)) if fields == arity => {
                        self.reuse_token = None;
                        Some(token)
                    }
                    _ => None,
                };
                let record = match reusable {
                    // A drop-reuse token of this shape: when it captured a
                    // block (the deconstructed record was uniquely owned),
                    // write the new header over it — its count is already
                    // one — and skip allocation entirely.
                    Some(token_slot) => {
                        let token = self.builder.use_var(token_slot);
                        let reused = self.builder.create_block();
                        let fresh = self.builder.create_block();
                        let joined = self.builder.create_block();
                        self.builder.append_block_param(joined, types::I64);
                        self.builder.ins().brif(token, reused, &[], fresh, &[]);
                        self.builder.seal_block(reused);
                        self.builder.seal_block(fresh);

                        self.builder.switch_to_block(reused);
                        let header_value =
                            self.builder.ins().iconst(types::I64, header as i64);
                        let _ = self.builder.ins().store(
                            MemFlagsData::trusted(),
                            header_value,
                            token,
                            0,
                        );
                        self.builder.ins().jump(joined, &[token.into()]);

                        self.builder.switch_to_block(fresh);
                        let allocated = self.pool_alloc(1 + arity, header, slow)?;
                        self.builder.ins().jump(joined, &[allocated.into()]);
                        self.builder.seal_block(joined);

                        self.builder.switch_to_block(joined);
                        self.builder.block_params(joined)[0]
                    }
                    None => self.pool_alloc(1 + arity, header, slow)?,
                };
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
        reuse: Option<ReuseCandidate>,
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
                    DecisionMode::Case { tail, .. } => {
                        // Drop-reuse: when this clause deconstructed one of
                        // the case's own subjects, is certain to end in a
                        // tail transfer (so the join path cannot release
                        // the subject again), and unconditionally constructs
                        // a record of the matched shape, the subject is
                        // released here instead of at the transfer — and a
                        // uniquely owned one keeps its memory as a token
                        // for that construction to reuse.
                        let mut tail = tail;
                        let mut claimed = false;
                        if let Some(candidate) = reuse
                            && self.reuse_token.is_none()
                            && let Some(chain) = &mut tail
                            && first_reuse_site(body, candidate.fields.len(), false)
                                == Some(true)
                            && self.body_transfers(body)
                            && let Some(position) = chain.iter().rposition(|(name, value)| {
                                name.is_none() && *value == candidate.subject
                            })
                        {
                            let _ = chain.remove(position);
                            let token = self.emit_reuse_claim(&candidate, variables)?;
                            self.reuse_token = Some((token, candidate.fields.len()));
                            claimed = true;
                        }
                        // Clause bindings join the scope machinery: the
                        // body's last-use analysis can consume them, and
                        // its scope release frees whatever survives. A
                        // transferring clause may also consume entries of
                        // enclosing scopes (their release moves into its
                        // arguments), which is per-path behaviour: restore
                        // the owned list afterwards so sibling clauses and
                        // the join path still see every entry.
                        let saved_owned = self.scope_owned.clone();
                        let scope_start = self.scope_owned.len();
                        self.scope_owned.extend(binding_slots.iter().cloned());
                        self.branch_depth += 1;
                        let outcome = self.statements_scoped(body, scope_start, tail);
                        self.branch_depth -= 1;
                        let outcome = outcome?;
                        if claimed && self.reuse_token.take().is_some() {
                            // The scan proved an unconditional construction
                            // consumes the token; reaching here with it
                            // still armed is a translator bug.
                            return Err("drop-reuse token was not consumed".into());
                        }
                        match outcome {
                            None => {
                                // The clause tail-transferred; nothing joins.
                                self.scope_owned = saved_owned;
                                if let Some(environment) = saved_environment {
                                    self.environment = environment;
                                }
                                return Ok(false);
                            }
                            Some(result) => {
                                self.scope_owned = saved_owned;
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
                    DecisionMode::Case {
                        tail: Some(outer), ..
                    } => {
                        let mut cleanups = outer.clone();
                        // Guard bindings are released outside the scope
                        // machinery, so their entries carry no name and are
                        // never consumed.
                        for slot in &binding_slots {
                            cleanups.push((None, self.builder.use_var(*slot)));
                        }
                        // A transferring body may consume enclosing scopes'
                        // entries; restore the owned list for the guard's
                        // false path and everything after (see the clause
                        // body case above).
                        let saved_owned = self.scope_owned.clone();
                        let scope_start = self.scope_owned.len();
                        self.branch_depth += 1;
                        let outcome = self.statements_scoped(if_true, scope_start, Some(cleanups));
                        self.branch_depth -= 1;
                        let outcome = outcome?;
                        self.scope_owned = saved_owned;
                        match outcome {
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
                        self.branch_depth += 1;
                        let result = self.statements(if_true);
                        self.branch_depth -= 1;
                        let result = result?;
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
                Ok(self.decision(variables, if_false, join, mode, None)? || jumped)
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
                // Only a case's own subject may be claimed for drop-reuse:
                // nested switches scrutinize fields borrowed from their
                // containers.
                let reusable_subject = matches!(
                    &mode,
                    DecisionMode::Case { tail: Some(_), subjects } if subjects.contains(&subject)
                );
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
                        joined |= self.decision(variables, decision, join, mode.clone(), None)?;
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
                        let candidate = if reusable_subject
                            && matches!(decision, native_ir::Decision::Run { .. })
                        {
                            Some(ReuseCandidate {
                                subject,
                                fields: extracted.clone(),
                            })
                        } else {
                            None
                        };
                        joined |=
                            self.decision(&extended, decision, join, mode.clone(), candidate)?;
                    }
                    self.builder.switch_to_block(next_block);
                }
                // An exhaustive match's final variant is not tag-tested,
                // but its fields still become decision variables.
                if fallback_fields.is_empty() {
                    Ok(self.decision(variables, fallback, join, mode, None)? || joined)
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
                    let candidate = if reusable_subject
                        && matches!(fallback.as_ref(), native_ir::Decision::Run { .. })
                    {
                        Some(ReuseCandidate {
                            subject,
                            fields: fallback_fields.clone(),
                        })
                    } else {
                        None
                    };
                    Ok(self.decision(&extended, fallback, join, mode, candidate)? || joined)
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
        let boxed = self.pool_alloc(2, native_runtime::KIND_FLOAT, |this| {
            let from_bits_ref = this
                .module
                .declare_func_in_func(this.runtime.float_from_bits, this.builder.func);
            let call = this.builder.ins().call(from_bits_ref, &[bits]);
            Ok(this.builder.inst_results(call)[0])
        })?;
        // The fallback stored the same payload; an unconditional store
        // keeps the block structure simple.
        let _ = self
            .builder
            .ins()
            .store(MemFlagsData::trusted(), bits, boxed, 8);
        Ok(boxed)
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

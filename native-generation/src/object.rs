// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! The ahead-of-time driver: compiles a set of native IR modules to an
//! object file for the host machine or a cross-compilation target, ready to
//! be linked with the `native-runtime-static` library into an executable.
//!
//! The object exports the same entry wrapper the JIT calls, plus a program
//! data blob holding what the JIT passes to the runtime in process: the
//! configured stack size and the interned constructor names. The static
//! library's C `main` decodes the blob and runs the wrapper.

use std::str::FromStr;

use std::collections::HashMap;

use cranelift_codegen::isa;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_module::{DataDescription, Linkage, Module, default_libcall_names};
use cranelift_object::object::write::{Relocation, StandardSegment};
use cranelift_object::object::{
    RelocationEncoding, RelocationFlags, RelocationKind, SectionKind,
};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use gimli::write::{
    Address, AttributeValue, DwarfUnit, EndianVec, LineProgram, LineString, Sections, Writer,
};
use gimli::{Encoding, Format, LineEncoding, LittleEndian};
use target_lexicon::Triple;

use crate::translate::{DebugFunction, Translator};

/// Compiles the given modules to an object file, with `main` in
/// `main_module` reachable through the exported entry wrapper. The object
/// targets the given triple, or the host machine when `triple` is `None`.
/// Returns the object file's bytes.
pub fn compile(
    modules: &[native_ir::Module],
    main_module: &str,
    stack_size_megabytes: u64,
    triple: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut flag_builder = settings::builder();
    // Position-independent code, so the executable can be linked PIE (the
    // default on modern macOS and Linux toolchains).
    flag_builder.set("is_pic", "true").expect("valid flag");
    flag_builder
        .set("opt_level", "speed")
        .expect("valid flag");
    let flags = settings::Flags::new(flag_builder);
    let isa = match triple {
        // The host builder detects the machine's CPU features; a foreign
        // triple gets the architecture's baseline features.
        None => cranelift_native::builder()
            .map_err(|error| format!("host machine is not supported: {error}"))?
            .finish(flags)
            .map_err(|error| error.to_string())?,
        Some(triple) => {
            let triple = Triple::from_str(triple)
                .map_err(|error| format!("invalid target triple `{triple}`: {error}"))?;
            isa::lookup(triple.clone())
                .map_err(|error| format!("target `{triple}` is not supported: {error}"))?
                .finish(flags)
                .map_err(|error| error.to_string())?
        }
    };

    let builder = ObjectBuilder::new(isa, "gleam", default_libcall_names())
        .map_err(|error| error.to_string())?;
    let mut object_module = ObjectModule::new(builder);

    let mut translator = Translator::new(&mut object_module)?;
    for module in modules {
        translator.declare_module(module)?;
    }
    for module in modules {
        translator.define_module(module)?;
    }
    let debug_functions = translator.take_debug_functions();

    let main = translator.function_id(main_module, "main").ok_or_else(|| {
        format!("module `{main_module}` has no `main` function compiled for the native target")
    })?;
    let _ = translator.define_entry_wrapper(main)?;

    let program_data = native_runtime::encode_program_data(
        stack_size_megabytes,
        &translator.constructor_names(),
    );
    let data = object_module
        .declare_data(native_runtime::PROGRAM_DATA_SYMBOL, Linkage::Export, false, false)
        .map_err(|error| error.to_string())?;
    let mut description = DataDescription::new();
    description.define(program_data.into_boxed_slice());
    object_module
        .define_data(data, &description)
        .map_err(|error| error.to_string())?;

    let mut product = object_module.finish();
    append_debug_info(&mut product, &debug_functions)?;
    product.emit().map_err(|error| error.to_string())
}

/// A DWARF section writer that records where symbol addresses land, so
/// they become object relocations the linker resolves.
#[derive(Clone)]
struct RelocationWriter {
    writer: EndianVec<LittleEndian>,
    /// (section offset, [`DebugFunction`] index) pairs needing an
    /// absolute address relocation.
    relocations: Vec<(u64, usize)>,
}

impl Writer for RelocationWriter {
    type Endian = LittleEndian;

    fn endian(&self) -> LittleEndian {
        LittleEndian
    }

    fn len(&self) -> usize {
        self.writer.len()
    }

    fn write(&mut self, bytes: &[u8]) -> gimli::write::Result<()> {
        self.writer.write(bytes)
    }

    fn write_at(&mut self, offset: usize, bytes: &[u8]) -> gimli::write::Result<()> {
        self.writer.write_at(offset, bytes)
    }

    fn write_address(&mut self, address: Address, size: u8) -> gimli::write::Result<()> {
        match address {
            Address::Constant(value) => self.writer.write_udata(value, size),
            Address::Symbol { symbol, addend } => {
                self.relocations.push((self.len() as u64, symbol));
                self.writer.write_udata(addend as u64, size)
            }
        }
    }
}

/// Appends DWARF line and function information to the object: one compile
/// unit whose line program maps each function's code (addressed through
/// its symbol, relocated at link time) to the source lines the translator
/// stamped on it. Debuggers and profilers then resolve Gleam frames to
/// `file:line`.
///
/// Cross-section offsets are written as plain values, which is correct
/// while this object is the only DWARF contributor to the executable —
/// the runtime's release static library carries no debug sections.
fn append_debug_info(
    product: &mut ObjectProduct,
    functions: &[DebugFunction],
) -> Result<(), String> {
    if functions.is_empty() {
        return Ok(());
    }
    let encoding = Encoding {
        format: Format::Dwarf32,
        version: 4,
        address_size: 8,
    };
    let mut dwarf = DwarfUnit::new(encoding);
    let mut line_program = LineProgram::new(
        encoding,
        LineEncoding::default(),
        LineString::String(b".".to_vec()),
        None,
        LineString::String(b"gleam".to_vec()),
        None,
    );
    let default_directory = line_program.default_directory();
    let mut file_ids = HashMap::new();
    for function in functions {
        let _ = file_ids
            .entry(function.src_path.as_str())
            .or_insert_with(|| {
                line_program.add_file(
                    LineString::String(function.src_path.clone().into_bytes()),
                    default_directory,
                    None,
                )
            });
    }
    for (index, function) in functions.iter().enumerate() {
        if function.rows.is_empty() {
            continue;
        }
        let file_id = file_ids[function.src_path.as_str()];
        line_program.begin_sequence(Some(Address::Symbol {
            symbol: index,
            addend: 0,
        }));
        for (offset, line) in &function.rows {
            line_program.row().address_offset = *offset as u64;
            line_program.row().file = file_id;
            line_program.row().line = *line as u64;
            line_program.generate_row();
        }
        line_program.end_sequence(function.code_length as u64);
    }
    dwarf.unit.line_program = line_program;

    let root = dwarf.unit.root();
    dwarf.unit.get_mut(root).set(
        gimli::DW_AT_producer,
        AttributeValue::String(b"gleam native".to_vec()),
    );
    dwarf
        .unit
        .get_mut(root)
        .set(gimli::DW_AT_name, AttributeValue::String(b"gleam".to_vec()));
    dwarf
        .unit
        .get_mut(root)
        .set(gimli::DW_AT_comp_dir, AttributeValue::String(b".".to_vec()));
    for (index, function) in functions.iter().enumerate() {
        let die = dwarf.unit.add(root, gimli::DW_TAG_subprogram);
        let entry = dwarf.unit.get_mut(die);
        entry.set(
            gimli::DW_AT_name,
            AttributeValue::String(function.name.clone().into_bytes()),
        );
        entry.set(
            gimli::DW_AT_low_pc,
            AttributeValue::Address(Address::Symbol {
                symbol: index,
                addend: 0,
            }),
        );
        entry.set(
            gimli::DW_AT_high_pc,
            AttributeValue::Udata(function.code_length as u64),
        );
        entry.set(
            gimli::DW_AT_decl_file,
            AttributeValue::FileIndex(Some(file_ids[function.src_path.as_str()])),
        );
    }

    let mut sections = Sections::new(RelocationWriter {
        writer: EndianVec::new(LittleEndian),
        relocations: Vec::new(),
    });
    dwarf
        .write(&mut sections)
        .map_err(|error| error.to_string())?;

    sections.for_each_mut(|id, writer| -> Result<(), String> {
        if writer.writer.len() == 0 {
            return Ok(());
        }
        let segment = product
            .object
            .segment_name(StandardSegment::Debug)
            .to_vec();
        // Mach-O spells ".debug_info" as "__debug_info" (in the __DWARF
        // segment); the object writer does not translate custom names.
        let name = match product.object.format() {
            cranelift_object::object::BinaryFormat::MachO => {
                id.name().replacen('.', "__", 1)
            }
            _ => id.name().to_string(),
        };
        let section =
            product
                .object
                .add_section(segment, name.into_bytes(), SectionKind::Debug);
        let offset = product
            .object
            .append_section_data(section, writer.writer.slice(), 1);
        for (relocation_offset, function_index) in &writer.relocations {
            // Section-relative, like a C compiler's debug relocations:
            // the well-trodden path through dsymutil and lldb. The
            // function's section offset becomes the addend against the
            // text section's own symbol.
            let function = product.function_symbol(functions[*function_index].id);
            let placement = {
                let definition = product.object.symbol(function);
                match definition.section {
                    cranelift_object::object::write::SymbolSection::Section(text) => {
                        Some((text, definition.value as i64))
                    }
                    _ => None,
                }
            };
            let (symbol, addend) = match placement {
                Some((text, value)) => (product.object.section_symbol(text), value),
                None => (function, 0),
            };
            product
                .object
                .add_relocation(
                    section,
                    Relocation {
                        offset: offset + relocation_offset,
                        symbol,
                        addend,
                        flags: RelocationFlags::Generic {
                            kind: RelocationKind::Absolute,
                            encoding: RelocationEncoding::Generic,
                            size: 64,
                        },
                    },
                )
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    })
}

use heck::ToKebabCase;
use std::{fs::File, io::Write, path::Path};
use valkyrie_wit::ForeignGenerator;
use wit_bindgen_core::wit_parser::{Enum, EnumCase, Field, Function, FunctionKind, Record, Results, Type, TypeDefKind};
use wit_component::ComponentEncoder;
use wit_parser::Handle;

mod codegen;
#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn parse_source() -> anyhow::Result<()> {
    let mut resolve = ForeignGenerator::new("core")?;
    let number = resolve.make_module(&["number".to_string()]);
    let sign = resolve.make_type(
        number,
        "Sign",
        TypeDefKind::Enum(Enum {
            cases: vec![
                EnumCase { name: "NoSign".to_kebab_case(), docs: Default::default() },
                EnumCase { name: "Positive".to_kebab_case(), docs: Default::default() },
                EnumCase { name: "Negative".to_kebab_case(), docs: Default::default() },
            ],
        }),
    );
    let nat = resolve.make_type(
        number,
        "Natural",
        TypeDefKind::Record(Record { fields: vec![Field { name: "gc".to_string(), ty: Type::U32, docs: Default::default() }] }),
    );
    let int = resolve.make_type(
        number,
        "Integer",
        TypeDefKind::Record(Record {
            fields: vec![
                Field { name: "sign".to_kebab_case(), ty: Type::Id(sign), docs: Default::default() },
                Field { name: "natural".to_kebab_case(), ty: Type::Id(nat), docs: Default::default() },
            ],
        }),
    );

    resolve.make_function(
        number,
        Function {
            name: "nat_new_u64".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("n".to_string(), Type::U64)],
            results: Results::Anon(Type::Id(nat)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "nat_add_u64".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(nat)), ("rhs".to_string(), Type::U64)],
            results: Results::Anon(Type::Id(nat)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "nat_add_u64_inplace".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(nat)), ("rhs".to_string(), Type::U64)],
            results: Results::Named(vec![]),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "nat_add_nat".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(nat)), ("rhs".to_string(), Type::Id(nat))],
            results: Results::Anon(Type::Id(nat)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "nat_add_nat_inplace".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(nat)), ("rhs".to_string(), Type::Id(nat))],
            results: Results::Named(vec![]),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "int_add_u64".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(int)), ("rhs".to_string(), Type::U64)],
            results: Results::Anon(Type::Id(int)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "int_add_nat".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(int)), ("rhs".to_string(), Type::Id(nat))],
            results: Results::Anon(Type::Id(int)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        number,
        Function {
            name: "int_add_int".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(int)), ("rhs".to_string(), Type::Id(int))],
            results: Results::Anon(Type::Id(int)),
            docs: Default::default(),
        },
    );

    let text = resolve.make_module(&["text".to_string()]);
    let ascii = resolve.make_type(
        text,
        "Ascii",
        TypeDefKind::Record(Record {
            fields: vec![Field { name: "_codepoint".to_kebab_case(), ty: Type::U8, docs: Default::default() }],
        }),
    );
    let unicode = resolve.make_type(
        text,
        "Unicode",
        TypeDefKind::Record(Record {
            fields: vec![Field { name: "_codepoint".to_kebab_case(), ty: Type::U32, docs: Default::default() }],
        }),
    );
    resolve.make_function(
        text,
        Function {
            name: "ascii_add_u8".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(ascii)), ("rhs".to_string(), Type::U8)],
            results: Results::Anon(Type::Id(ascii)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        text,
        Function {
            name: "unicode_add_u32".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("this".to_string(), Type::Id(unicode)), ("rhs".to_string(), Type::U32)],
            results: Results::Anon(Type::Id(unicode)),
            docs: Default::default(),
        },
    );

    // for (_, item) in &resolve.worlds {
    //     println!("Worlds: {:?}", item.name);
    // }
    // for (_, item) in &resolve.types {
    //     println!("Types: {:?}", item);
    // }

    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    resolve.build_rust(&here.join("tests/codegen"))?;
    // resolve.build_java(&here.join("tests/codegen"))?;
    // resolve.build_csharp(&here.join("tests/codegen"))?;
    Ok(())
}

#[test]
fn decode_wasm() -> anyhow::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    let debug = here.join("../../target/wasm32-wasi/release/").canonicalize()?;
    let input = std::fs::read(debug.join("valkyrie_ffi.wasm"))?;
    let mut out = File::create(debug.join("valkyrie-wit.wasm"))?;

    let mut encoder = ComponentEncoder::default().validate(true).module(&input)?;
    encoder = encoder.adapter("wasi_snapshot_preview1", include_bytes!("wasi_snapshot_preview1.wasm"))?;

    encoder = encoder.realloc_via_memory_grow(true);

    let bytes = encoder.encode()?;
    out.write_all(&bytes)?;
    Ok(())
}

use std::path::Path;
use valkyrie_wit::ForeignGenerator;
use wit_bindgen_core::wit_parser::{Enum, Field, Function, FunctionKind, Record, Results, Type, TypeDefKind};

mod codegen;
#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn parse_source2() -> anyhow::Result<()> {
    let mut resolve = ForeignGenerator::new("native")?;
    let mid = resolve.make_module("number");
    let sign = resolve.make_type(mid, "Sign", TypeDefKind::Enum(Enum { cases: vec![] }));
    let nat = resolve.make_type(mid, "Natural", TypeDefKind::List(Type::U8));
    let int = resolve.make_type(
        mid,
        "Integer",
        TypeDefKind::Record(Record {
            fields: vec![
                Field { name: "sign".to_string(), ty: Type::Id(sign), docs: Default::default() },
                Field { name: "natural".to_string(), ty: Type::Id(sign), docs: Default::default() },
            ],
        }),
    );
    // let t_nat = resolve.get_module();
    resolve.make_function(
        mid,
        Function {
            name: "recast".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("self".to_string(), Type::Id(nat)), ("rhs".to_string(), Type::Id(nat))],
            results: Results::Anon(Type::Id(int)),
            docs: Default::default(),
        },
    );
    resolve.make_function(
        mid,
        Function {
            name: "new".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("self".to_string(), Type::Id(nat)), ("rhs".to_string(), Type::Id(nat))],
            results: Results::Anon(Type::Id(int)),
            docs: Default::default(),
        },
    );

    let mid = resolve.make_module("text");
    let utf = resolve.make_type(mid, "Natural", TypeDefKind::List(Type::U8));
    resolve.make_function(
        mid,
        Function {
            name: "recast".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![("self".to_string(), Type::Id(utf)), ("rhs".to_string(), Type::Id(utf))],
            results: Results::Anon(Type::Id(int)),
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
    resolve.build_markdown(&here.join("tests/codegen"))?;
    Ok(())
}

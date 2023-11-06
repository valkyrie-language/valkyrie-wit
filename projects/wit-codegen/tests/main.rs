use convert_case::{Case, Casing};
use std::path::Path;
use valkyrie_wit::ForeignGenerator;
use wit_bindgen_core::wit_parser::{Enum, EnumCase, Field, Function, FunctionKind, Record, Results, Type, TypeDefKind};

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
                EnumCase { name: "NoSign".to_case(Case::Kebab), docs: Default::default() },
                EnumCase { name: "Positive".to_case(Case::Kebab), docs: Default::default() },
                EnumCase { name: "Negative".to_case(Case::Kebab), docs: Default::default() },
            ],
        }),
    );
    let nat = resolve.make_type(number, "Natural", TypeDefKind::Record(Record { fields: vec![] }));
    let int = resolve.make_type(
        number,
        "Integer",
        TypeDefKind::Record(Record {
            fields: vec![
                Field { name: "sign".to_string(), ty: Type::Id(sign), docs: Default::default() },
                Field { name: "natural".to_string(), ty: Type::Id(nat), docs: Default::default() },
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

    // for (_, item) in &resolve.worlds {
    //     println!("Worlds: {:?}", item.name);
    // }
    // for (_, item) in &resolve.types {
    //     println!("Types: {:?}", item);
    // }

    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    resolve.build_rust(&here.join("tests/codegen"))?;
    resolve.build_markdown(&here.join("tests/codegen"))?;
    // resolve.build_java(&here.join("tests/codegen"))?;
    // resolve.build_csharp(&here.join("tests/codegen"))?;
    Ok(())
}

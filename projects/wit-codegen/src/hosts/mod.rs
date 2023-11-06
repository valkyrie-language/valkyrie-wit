use crate::helpers::{find_type, take_interface, take_world, PackageResolver, WitPackage};

use id_arena::Id;
use std::{collections::HashMap, fs::File, io::Write, path::Path};
use wit_bindgen_core::{
    wit_parser::{
        Field, Function, FunctionKind, Interface, PackageId, Record, Resolve, Results, Type, TypeDef, TypeDefKind, TypeOwner,
        UnresolvedPackage, WorldItem, WorldKey,
    },
    Files,
};
use wit_bindgen_rust::{ExportKey, Opts};

#[test]
fn test() {
    let _ = parse_source2();
}

fn parse_source() -> anyhow::Result<(Resolve, PackageId)> {
    let mut resolve = WitPackage::new("native")?;
    let mut resolve = Resolve::default();
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    let root = UnresolvedPackage::parse(&Path::new("<anonymous>"), "package v:native;world native {}").unwrap();
    let pid = resolve.push(root).unwrap();

    let t_nat = resolve.make_type(pid, "number", "Natural");
    let (i_id, i) = take_interface(&mut resolve, pid, "number");
    i.functions.insert(
        "test".to_string(),
        Function {
            name: "test".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            results: Results::Anon(t_nat),
            docs: Default::default(),
        },
    );

    let (w_id, w) = take_world(&mut resolve, pid, "native");
    w.exports.insert(WorldKey::Interface(i_id), WorldItem::Interface(i_id));

    for (_, item) in &resolve.worlds {
        println!("Worlds: {:?}", item.name);
    }
    for (_, item) in &resolve.types {
        println!("Types: {:?}", item);
    }
    let mut exports = HashMap::<ExportKey, String>::default();
    exports.insert(ExportKey::Name("v:native/my-plugin-api".to_string()), "Host1".to_string());
    exports.insert(ExportKey::Name("v:native/number".to_string()), "Host2".to_string());
    // exports.insert(ExportKey::Name("vit:number/integer".to_string()), "Host3".to_string());
    // exports.insert(ExportKey::Name("vit:number/ordinal".to_string()), "Host4".to_string());
    // exports.insert(ExportKey::Name("vit:number/decimal".to_string()), "Host5".to_string());
    // exports.insert(ExportKey::Name("vit:number/fraction".to_string()), "Host6".to_string());
    // exports.insert(ExportKey::Name("vit:number/prime".to_string()), "Host7".to_string());
    let mut builder = Opts { rustfmt: false, exports, ..Default::default() }.build();
    let mut files = Files::default();
    let world = resolve.select_world(pkg, Some("number-ffi"))?;
    match resolve.worlds.get(world) {
        None => {}
        Some(s) => {
            println!("{s:#?}")
        }
    }

    builder.generate(&resolve, world, &mut files).unwrap();

    for (name, content) in files.iter() {
        let mut file = File::create(here.join("tests").join("codegen").join(name))?;
        file.write_all(content)?;
    }

    Ok((resolve, pkg))
}

fn parse_source2() -> anyhow::Result<()> {
    let mut resolve = WitPackage::new("native")?;
    let t_nat = resolve.make_type(
        "number",
        "Natural",
        TypeDefKind::Record(Record {
            fields: vec![Field { name: "check".to_string(), ty: Type::Bool, docs: Default::default() }],
        }),
    );

    let (i_id, i) = take_interface(&mut resolve, pid, "number");
    i.functions.insert(
        "test".to_string(),
        Function {
            name: "test".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            results: Results::Anon(t_nat),
            docs: Default::default(),
        },
    );

    let (w_id, w) = take_world(&mut resolve, pid, "native");
    w.exports.insert(WorldKey::Interface(i_id), WorldItem::Interface(i_id));

    for (_, item) in &resolve.worlds {
        println!("Worlds: {:?}", item.name);
    }
    for (_, item) in &resolve.types {
        println!("Types: {:?}", item);
    }

    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;

    resolve.build_rust(&here.join("tests/codegen"))?;

    Ok(())
}

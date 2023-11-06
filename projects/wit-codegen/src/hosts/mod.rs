use crate::helpers::{take_interface, take_world};
use id_arena::Id;
use std::{collections::HashMap, fs::File, io::Write, path::Path};
use wit_bindgen_core::{
    wit_parser::{
        Function, FunctionKind, Interface, Package, PackageId, Resolve, Results, Type, TypeDef, TypeDefKind, TypeOwner,
        UnresolvedPackage, World, WorldItem, WorldKey,
    },
    Files,
};
use wit_bindgen_rust::{ExportKey, Opts};

#[test]
fn test() {
    let _ = parse_source2();
}

fn parse_source() -> anyhow::Result<(Resolve, PackageId)> {
    let mut resolve = Resolve::default();
    // let mut files = Vec::new();
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;

    let (pkg, _) = resolve.push_dir(&here.join("wit"))?;
    //
    let mut exports: HashMap<ExportKey, String> = Default::default();
    exports.insert(ExportKey::Name("vit:number/natural/natural".to_string()), "Host1".to_string());
    exports.insert(ExportKey::Name("vit:number/integer".to_string()), "Host2".to_string());
    exports.insert(ExportKey::Name("vit:number/integer".to_string()), "Host3".to_string());
    exports.insert(ExportKey::Name("vit:number/ordinal".to_string()), "Host4".to_string());
    exports.insert(ExportKey::Name("vit:number/decimal".to_string()), "Host5".to_string());
    exports.insert(ExportKey::Name("vit:number/fraction".to_string()), "Host6".to_string());
    exports.insert(ExportKey::Name("vit:number/prime".to_string()), "Host7".to_string());

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

fn parse_source2() -> anyhow::Result<(Resolve, PackageId)> {
    let mut resolve = Resolve::default();
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    let root = UnresolvedPackage::parse(
        &Path::new("<anonymous>"),
        "package v:native;
interface my-plugin-api {
  record coord {
    x: u32,
    y: u32,
  }

  get-position: func() -> coord
  set-position: func(pos: coord)

  record monster {
    name: string,
    hp: u32,
    pos: coord,
  }

  monsters: func() -> list<monster>
}

world native {
    import print: func(msg: string)
    export my-plugin-api
}
",
    )
    .unwrap();
    let pid = resolve.push(root).unwrap();

    let (i_id, i) = take_interface(&mut resolve, pid, "number");

    i.functions.insert(
        "take".to_string(),
        Function {
            name: "test".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            results: Results::Anon(Type::Char),
            docs: Default::default(),
        },
    );

    let (w_id, w) = take_world(&mut resolve, pid, "native");

    let t_nat = resolve.types.alloc(TypeDef {
        name: Some("Natural".to_string()),
        kind: TypeDefKind::Resource,
        owner: TypeOwner::Interface(i_id),
        docs: Default::default(),
    });

    // w_number.exports.insert(WorldKey::Interface(i_int), WorldItem::Type(t_nat));

    for (_, item) in &resolve.worlds {
        println!("Worlds: {:?}", item.name);
    }
    for (_, item) in &resolve.types {
        println!("Types: {:?}", item);
    }
    let mut exports = HashMap::<ExportKey, String>::default();
    exports.insert(ExportKey::Name("v:native/my-plugin-api".to_string()), "Host1".to_string());
    // exports.insert(ExportKey::Name("vit:number/integer".to_string()), "Host2".to_string());
    // exports.insert(ExportKey::Name("vit:number/integer".to_string()), "Host3".to_string());
    // exports.insert(ExportKey::Name("vit:number/ordinal".to_string()), "Host4".to_string());
    // exports.insert(ExportKey::Name("vit:number/decimal".to_string()), "Host5".to_string());
    // exports.insert(ExportKey::Name("vit:number/fraction".to_string()), "Host6".to_string());
    // exports.insert(ExportKey::Name("vit:number/prime".to_string()), "Host7".to_string());
    let mut builder = Opts { rustfmt: false, exports, ..Default::default() }.build();

    let mut files = Files::default();
    let world = resolve.select_world(pid, Some("native"))?;
    builder.generate(&resolve, world, &mut files).unwrap();

    for (name, content) in files.iter() {
        let mut file = File::create(here.join("tests").join("codegen").join(name))?;
        file.write_all(content)?;
    }

    Ok((resolve, pid))
}

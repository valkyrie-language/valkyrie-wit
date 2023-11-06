use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use wit_bindgen_core::{
    wit_parser::{PackageId, Resolve, World},
    Files,
};
use wit_bindgen_rust::{ExportKey, Opts};

#[test]
fn test() {
    let _ = parse_source();
}

fn parse_source() -> anyhow::Result<(Resolve, PackageId)> {
    let mut resolve = Resolve::default();
    // let mut files = Vec::new();
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;

    let (pkg, _) = resolve.push_dir(&here.join("wit"))?;
    //
    for (_, item) in &resolve.packages {
        println!("{:?}", item.name);
    }
    resolve.worlds.alloc(World {
        name: "text-ffi".to_string(),
        imports: Default::default(),
        exports: Default::default(),
        package: None,
        docs: Default::default(),
        includes: vec![],
        include_names: vec![],
    });
    //
    for (_, item) in &resolve.worlds {
        println!("Worlds: {:?}", item.name);
    }

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
    builder.generate(&resolve, world, &mut files).unwrap();

    for (name, content) in files.iter() {
        let mut file = File::create(here.join("tests").join("codegen").join(name))?;
        file.write_all(content)?;
    }

    Ok((resolve, pkg))
}

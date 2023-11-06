use std::path::{Path, PathBuf};
use wit_bindgen_core::{
    wit_parser::{PackageId, Resolve, UnresolvedPackage},
    Files, Source,
};
use wit_bindgen_rust::Opts;

#[test]
fn test() {
    parse_source();
}

fn parse_source() -> anyhow::Result<(Resolve, PackageId)> {
    let mut a = Opts::default().build();
    let mut resolve = Resolve::default();
    // let mut files = Vec::new();
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let (pkg, filebuf) = resolve.push_dir(&root.join("wit"))?;
    let mut files = Files::default();
    for i in filebuf {
        println!("Found: {:?}", i);
        files.push(&i.as_os_str().to_string_lossy(), i.as_os_str().as_encoded_bytes())
    }
    //
    for (_, item) in &resolve.packages {
        println!("{:?}", item.name);
    }
    //
    for (_, item) in &resolve.worlds {
        println!("{:?}", item.name);
    }

    //
    let world = resolve.select_world(pkg, Some("image-ffi"))?;
    println!("{:?}", world);
    a.generate(&resolve, world, &mut files)?;
    a.finish(&resolve, world, &mut files);

    Ok((resolve, pkg))
}

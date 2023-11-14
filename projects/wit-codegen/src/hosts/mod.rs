use heck::{ToKebabCase, ToUpperCamelCase};
use id_arena::Id;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use wit_bindgen_core::{
    wit_parser::{
        Function, Interface, Package, Resolve, TypeDef, TypeDefKind, TypeOwner, UnresolvedPackage, World, WorldItem, WorldKey,
    },
    Files,
};
use wit_bindgen_rust::ExportKey;

const LANGUAGE_ID: &'static str = "v";

/// Foreign Function Interface Generator for Valkyrie Language
pub struct ForeignGenerator {
    package_name: String,
    pid: Id<Package>,
    wid: Id<World>,
    map_module: HashMap<String, Id<Interface>>,
    resolve: Resolve,
}

impl ForeignGenerator {
    /// Create a new generator
    pub fn new(package: &str) -> anyhow::Result<Self> {
        let package = package.to_kebab_case();
        let mut resolve = Resolve::default();
        let root = UnresolvedPackage::parse(
            &Path::new("<anonymous>"),
            &format!("package {LANGUAGE_ID}:{package};world {package} {{}}"),
        )?;
        let pid = resolve.push(root)?;
        for (wid, w) in &resolve.worlds {
            if w.name.eq(&package) {
                return Ok(Self { package_name: package, pid, wid, map_module: Default::default(), resolve });
            }
        }
        unreachable!()
    }
    /// Get the language name and package infos
    pub fn clear_language(&mut self) {
        let p = self.resolve.packages.get_mut(self.pid).expect("");
        p.worlds.clear();
    }
    /// Get the package name and world infos
    pub fn get_package(&self) -> &World {
        self.resolve.worlds.get(self.wid).expect("")
    }
    pub fn make_module(&mut self, target: &[String]) -> Id<Interface> {
        let target = target.iter().map(|v| v.to_kebab_case()).join("/");
        if let Some(s) = self.map_module.get(&target) {
            return *s;
        }
        let mid = self.resolve.interfaces.alloc(Interface {
            name: Some(target.clone()),
            types: Default::default(),
            functions: Default::default(),
            docs: Default::default(),
            package: Some(self.pid),
        });
        self.map_module.insert(target, mid);
        let world = self.resolve.worlds.get_mut(self.wid).unwrap();
        world.exports.insert(WorldKey::Interface(mid), WorldItem::Interface(mid));
        mid
    }
    pub fn make_function(&mut self, mid: Id<Interface>, mut f: Function) {
        let name = f.name.to_kebab_case();
        f.name = name.clone();
        let module = self.mut_module(mid);
        module.functions.insert(name, f);
    }
    pub fn get_module(&self, id: Id<Interface>) -> &Interface {
        self.resolve.interfaces.get(id).expect("")
    }
    pub fn mut_module(&mut self, id: Id<Interface>) -> &mut Interface {
        self.resolve.interfaces.get_mut(id).expect("")
    }
    pub fn make_type(&mut self, mid: Id<Interface>, name: &str, kind: TypeDefKind) -> Id<TypeDef> {
        let tid = self.resolve.types.alloc(TypeDef {
            name: Some(name.to_kebab_case()),
            kind,
            owner: TypeOwner::Interface(mid),
            docs: Default::default(),
        });
        let module = self.mut_module(mid);
        module.types.insert(name.to_string(), tid);
        tid
    }
    pub fn anonymous_type(&mut self, kind: TypeDefKind) -> Id<TypeDef> {
        let tid = self.resolve.types.alloc(TypeDef { name: None, kind, owner: TypeOwner::None, docs: Default::default() });
        // let module = self.mut_module(mid);
        // module.types.insert(name.to_string(), tid);
        tid
    }
    pub fn get_type(&self, id: Id<TypeDef>) -> Option<&TypeDef> {
        self.resolve.types.get(id)
    }
    pub fn mut_type(&mut self, id: Id<TypeDef>) -> Option<&mut TypeDef> {
        self.resolve.types.get_mut(id)
    }
}

impl ForeignGenerator {
    pub fn build_rust<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        let path = ensure_dir(dir)?;
        let mut builder =
            wit_bindgen_rust::Opts { rustfmt: false, exports: self.ensure_export(), ..Default::default() }.build();

        let mut files = Files::default();
        builder.generate(&self.resolve, self.wid, &mut files)?;
        for (name, content) in files.iter() {
            let mut file = File::create(path.join(name))?;
            file.write_all(content)?;
        }
        Ok(())
    }
    pub fn build_csharp<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        let path = ensure_dir(dir)?;
        let mut builder = wit_bindgen_csharp::Opts::default().build();
        let mut files = Files::default();
        builder.generate(&self.resolve, self.wid, &mut files)?;
        for (name, content) in files.iter() {
            let mut file = File::create(path.join(name))?;
            file.write_all(content)?;
        }
        Ok(())
    }
    pub fn build_java<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        let path = ensure_dir(dir)?;
        let mut builder = wit_bindgen_teavm_java::Opts::default().build();
        let mut files = Files::default();
        builder.generate(&self.resolve, self.wid, &mut files)?;
        for (name, content) in files.iter() {
            let mut file = File::create(path.join(name))?;
            file.write_all(content)?;
        }
        Ok(())
    }
    fn ensure_export(&self) -> HashMap<ExportKey, String> {
        let mut exports = HashMap::default();
        for (_, i) in &self.get_package().exports {
            match i {
                WorldItem::Interface(i) => match &self.get_module(*i).name {
                    None => {}
                    Some(s) => {
                        exports.insert(
                            ExportKey::Name(format!("{}:{}/{}", LANGUAGE_ID, self.package_name, s)),
                            format!("{}FFI", s.to_upper_camel_case()),
                        );
                    }
                },
                WorldItem::Function(_) => {}
                WorldItem::Type(v) => {
                    println!("{:?}", self.get_type(*v))
                }
            }
        }
        for (_, ty) in &self.resolve.types {
            match get_type_namepath(self, ty) {
                Some((m_name, name)) => {
                    exports.insert(
                        ExportKey::Name(format!("{}:{}/{}/{}", LANGUAGE_ID, self.package_name, m_name, name)),
                        format!("{}FFI", name),
                    );
                }
                _ => {}
            }
        }
        exports
    }
}

fn ensure_dir<'i, P>(path: P) -> anyhow::Result<PathBuf>
where
    P: AsRef<Path> + 'i,
{
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    if !path.is_dir() {
        anyhow::bail!("{} is not a directory", path.display());
    }
    Ok(path.canonicalize()?)
}

fn get_type_namepath<'a>(this: &'a ForeignGenerator, ty: &'a TypeDef) -> Option<(&'a str, &'a str)> {
    let name = ty.name.as_ref()?;
    match &ty.owner {
        TypeOwner::Interface(i) => {
            let m = this.get_module(*i);
            let m_name = m.name.as_ref()?;
            Some((m_name, name))
        }
        _ => None,
    }
}

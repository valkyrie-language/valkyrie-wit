use id_arena::Id;
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use wit_bindgen_core::{
    wit_parser::{
        Enum, Field, Function, FunctionKind, Interface, Package, Record, Resolve, Results, Type, TypeDef, TypeDefKind,
        TypeOwner, UnresolvedPackage, World, WorldItem, WorldKey,
    },
    Files,
};
use wit_bindgen_rust::ExportKey;

pub struct ForeignGenerator {
    package_name: String,
    pid: Id<Package>,
    wid: Id<World>,
    map_module: HashMap<String, Id<Interface>>,
    resolve: Resolve,
}

const LANGUAGE_ID: &'static str = "v";

impl ForeignGenerator {
    pub fn new(package: &str) -> anyhow::Result<Self> {
        let mut resolve = Resolve::default();
        let root = UnresolvedPackage::parse(
            &Path::new("<anonymous>"),
            &format!("package {LANGUAGE_ID}:{package};world {package} {{}}"),
        )?;
        let pid = resolve.push(root)?;
        for (wid, w) in &resolve.worlds {
            if w.name.eq(package) {
                return Ok(Self { package_name: package.to_string(), pid, wid, map_module: Default::default(), resolve });
            }
        }
        unreachable!()
    }
    /// Get the language name and package infos
    pub fn get_language(&self) -> &Package {
        self.resolve.packages.get(self.pid).expect("")
    }
    /// Get the package name and world infos
    pub fn get_package(&self) -> &World {
        self.resolve.worlds.get(self.wid).expect("")
    }
    pub fn make_module(&mut self, target: &str) -> Id<Interface> {
        match self.map_module.get(target) {
            None => {}
            Some(s) => {
                return *s;
            }
        }
        let mid = self.resolve.interfaces.alloc(Interface {
            name: Some(target.to_string()),
            types: Default::default(),
            functions: Default::default(),
            docs: Default::default(),
            package: Some(self.pid),
        });
        self.map_module.insert(target.to_string(), mid);
        let world = self.resolve.worlds.get_mut(self.wid).unwrap();
        world.exports.insert(WorldKey::Interface(mid), WorldItem::Interface(mid));
        mid
    }
    pub fn make_function(&mut self, mid: Id<Interface>, f: Function) {
        let module = self.mut_module(mid);
        module.functions.insert(f.name.clone(), f);
    }
    pub fn get_module(&self, id: Id<Interface>) -> &Interface {
        self.resolve.interfaces.get(id).expect("")
    }
    pub fn mut_module(&mut self, id: Id<Interface>) -> &mut Interface {
        self.resolve.interfaces.get_mut(id).expect("")
    }
    pub fn make_type(&mut self, mid: Id<Interface>, name: &str, kind: TypeDefKind) -> Id<TypeDef> {
        let tid = self.resolve.types.alloc(TypeDef {
            name: Some(name.to_string()),
            kind,
            owner: TypeOwner::Interface(mid),
            docs: Default::default(),
        });
        let module = self.mut_module(mid);
        module.types.insert(name.to_string(), tid);
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
            wit_bindgen_rust::Opts { rustfmt: false, stubs: true, exports: self.ensure_export(), ..Default::default() }.build();
        let mut files = Files::default();
        builder.generate(&self.resolve, self.wid, &mut files)?;
        for (name, content) in files.iter() {
            let mut file = File::create(path.join(name))?;
            file.write_all(content)?;
        }
        Ok(())
    }
    pub fn build_markdown<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        let path = ensure_dir(dir)?;
        let mut builder = wit_bindgen_markdown::Opts::default().build();
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
                        exports.insert(ExportKey::Name(format!("{}:{}/{}", LANGUAGE_ID, self.package_name, s)), s.to_string());
                    }
                },
                WorldItem::Function(_) => {}
                WorldItem::Type(v) => {
                    println!("{:?}", self.get_type(*v))
                }
            }
        }
        exports.insert(ExportKey::Name("v:core/number/Natural".to_string()), "Natural".to_string());
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

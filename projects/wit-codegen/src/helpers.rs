use id_arena::Id;
use std::{collections::HashMap, fs::File, io::Write, path::Path};
use wit_bindgen_core::{
    wit_parser::{
        Interface, Package, Record, Resolve, Type, TypeDef, TypeDefKind, TypeOwner, UnresolvedPackage, World, WorldItem,
        WorldKey,
    },
    Files,
};
use wit_bindgen_rust::ExportKey;

pub trait PackageResolver {
    fn make_type(&mut self, pid: Id<Package>, module: &str, name: &str, kind: TypeDefKind) -> Type;

    fn get_type(&self, target: &str) -> Option<Type>;
    fn make_module(&mut self, pid: Id<Package>, path: &str) -> Id<Interface>;
}

pub struct WitPackage {
    pid: Id<Package>,
    wid: Id<World>,
    map_module: HashMap<String, Id<Interface>>,
    resolve: Resolve,
}

impl WitPackage {
    pub fn new(package: &str) -> anyhow::Result<Self> {
        let mut resolve = Resolve::default();
        let root = UnresolvedPackage::parse(&Path::new("<anonymous>"), &format!("package v:{package};world {package} {{}}"))?;
        let pid = resolve.push(root)?;
        for (wid, w) in &resolve.worlds {
            if w.name.eq(package) {
                return Ok(Self { pid, wid, map_module: Default::default(), resolve });
            }
        }
        unreachable!()
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
    pub fn get_module(&self, id: Id<Interface>) -> &Interface {
        self.resolve.interfaces.get(id).expect("")
    }
    pub fn mut_module(&mut self, id: Id<Interface>) -> &mut Interface {
        self.resolve.interfaces.get_mut(id).expect("")
    }
    pub fn make_type(&mut self, module: &str, name: &str, kind: TypeDefKind) -> Type {
        let mid = self.make_module(module);
        let tid = self.resolve.types.alloc(TypeDef {
            name: Some(name.to_string()),
            kind,
            owner: TypeOwner::Interface(mid),
            docs: Default::default(),
        });
        let module = self.mut_module(mid);
        module.types.insert(name.to_string(), tid);
        Type::Id(tid)
    }
}

impl WitPackage {
    pub fn build_rust<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        let path = dir.as_ref();
        if !path.exists() {
            std::fs::create_dir_all(dir)?;
        }
        if !path.is_dir() {
            anyhow::bail!("{} is not a directory", dir.display());
        }

        let mut exports = HashMap::<ExportKey, String>::default();
        exports.insert(ExportKey::Name("v:native/my-plugin-api".to_string()), "Host1".to_string());
        exports.insert(ExportKey::Name("v:native/number".to_string()), "Host2".to_string());

        let mut builder = wit_bindgen_rust::Opts { rustfmt: false, exports, ..Default::default() }.build();
        let mut files = Files::default();
        builder.generate(&self.resolve, self.wid, &mut files)?;
        for (name, content) in files.iter() {
            let mut file = File::create(path.join(name))?;
            file.write_all(content)?;
        }
        Ok(())
    }
}
impl PackageResolver for Resolve {
    fn make_type(&mut self, pid: Id<Package>, module: &str, name: &str, kind: TypeDefKind) -> Type {
        let mid = self.make_module(pid, module);
        let tid = self.types.alloc(TypeDef {
            name: Some(name.to_string()),
            kind: TypeDefKind::Record(Record { fields: vec![] }),
            owner: TypeOwner::Interface(mid),
            docs: Default::default(),
        });
        let module = self.interfaces.get_mut(mid).expect("");
        module.types.insert(name.to_string(), tid);
        Type::Id(tid)
    }

    fn get_type(&self, target: &str) -> Option<Type> {
        for (w, name) in self.types.iter() {
            match &name.name {
                Some(s) if s.eq(target) => {
                    return Some(Type::Id(w));
                }
                _ => {}
            }
        }
        None
    }

    fn make_module(&mut self, pid: Id<Package>, path: &str) -> Id<Interface> {
        for (w, name) in self.interfaces.iter() {
            match &name.name {
                Some(s) if s.eq(path) => {
                    return w;
                }
                _ => {}
            }
        }
        self.interfaces.alloc(Interface {
            name: Some(path.to_string()),
            types: Default::default(),
            functions: Default::default(),
            docs: Default::default(),
            package: Some(pid),
        })
    }
}

pub fn take_world<'a, 'b>(scope: &'a mut Resolve, pid: Id<Package>, name: &'b str) -> (Id<World>, &'a mut World) {
    let id = ensure_world(scope, pid, name);
    let o = scope.worlds.get_mut(id).expect("");
    (id, o)
}

fn ensure_world(scope: &mut Resolve, pid: Id<Package>, target: &str) -> Id<World> {
    for (w, name) in scope.worlds.iter_mut() {
        if name.name.eq(target) {
            return w;
        }
    }
    scope.worlds.alloc(World {
        name: target.to_string(),
        imports: Default::default(),
        exports: Default::default(),
        package: Some(pid),
        docs: Default::default(),
        includes: vec![],
        include_names: vec![],
    })
}

pub fn take_interface<'i>(scope: &'i mut Resolve, pid: Id<Package>, name: &str) -> (Id<Interface>, &'i mut Interface) {
    let id = ensure_interface(scope, pid, name);
    let o = scope.interfaces.get_mut(id).expect("");
    (id, o)
}

fn ensure_interface(scope: &mut Resolve, pid: Id<Package>, target: &str) -> Id<Interface> {
    for (w, object) in scope.interfaces.iter_mut() {
        match &object.name {
            Some(s) if s.eq(target) => {
                return w;
            }
            _ => {}
        }
    }
    scope.interfaces.alloc(Interface {
        name: Some(target.to_string()),
        types: Default::default(),
        functions: Default::default(),
        docs: Default::default(),
        package: Some(pid),
    })
}

pub fn find_type(scope: &Resolve, target: &str) -> Option<Type> {
    for (w, name) in scope.types.iter() {
        match &name.name {
            Some(s) if s.eq(target) => {
                return Some(Type::Id(w));
            }
            _ => {}
        }
    }
    None
}

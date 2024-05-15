use crate::helpers::{FunctionContext, TypeContext, WriteDefine, WriteReference};
use convert_case::{Case, Casing};
use std::{fmt::Write, io::Write as _, path::Path};
use wit_parser::{Function, FunctionKind, Handle, Interface, Package, Resolve, Results, Type, TypeDef, TypeDefKind, TypeId};

mod visit_types;

pub struct ValkyrieFFI {
    cache: Resolve,
}

impl ValkyrieFFI {
    pub fn new<P: AsRef<Path>>(directory: P) -> anyhow::Result<Self> {
        let mut resolved = Resolve::new();
        resolved.push_dir(directory.as_ref())?;
        Ok(Self { cache: resolved })
    }
    pub fn generate<P: AsRef<Path>>(&self, output: P) -> std::io::Result<()> {
        let output = output.as_ref();
        if !output.is_dir() {
            panic!("")
        }
        if !output.exists() {}
        for (_, item) in self.cache.packages.iter() {
            self.export_packages(item, output)?;
        }
        Ok(())
    }
    fn export_packages(&self, package: &Package, root: &Path) -> std::io::Result<()> {
        let org = package.name.namespace.as_str();
        let pkg = package.name.name.as_str();
        tracing::info!("exporting interface: {}/{}", org, pkg);
        let file = root.join(format!("{}/{}.vk", org, pkg));
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = std::fs::File::create(file)?;
        for (name, ty) in package.interfaces.iter() {
            match self.cache.interfaces.get(*ty) {
                Some(s) => {
                    let mut buffer = String::new();
                    self.export_interface(s, package, &mut buffer).unwrap();
                    file.write_all(buffer.as_bytes())?;
                }
                None => tracing::error!("interface not found: {:?}", name),
            }
        }
        Ok(())
    }
    fn export_interface<W: Write>(&self, interface: &Interface, package: &Package, file: &mut W) -> std::fmt::Result {
        let name = match interface.name.as_ref() {
            Some(s) => s,
            None => panic!("missing name"),
        };
        let org = package.name.namespace.as_str();
        let pkg = package.name.name.as_str();
        let namespace = format!("{}:{}/{}", org, pkg, name);
        for (name, item) in interface.types.iter() {
            match self.cache.types.get(*item) {
                Some(s) => {
                    if let Err(e) = s.kind.write_define(
                        file,
                        TypeContext { ffi: &self, interface, namespace: &namespace, wasi_name: "", def: &s },
                    ) {
                        tracing::error!("error exporting type: {:?}", e)
                    }
                }
                None => tracing::error!("type not found: {:?}", name),
            }
        }
        for (_, item) in interface.functions.iter() {
            item.write_define(file, FunctionContext { ffi: &self, class_name: "", namespace: &namespace })?
        }
        Ok(())
    }
}

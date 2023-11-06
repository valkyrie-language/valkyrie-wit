use id_arena::Id;
use wit_bindgen_core::wit_parser::{Interface, Package, Resolve, World};

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
    match find_interface(scope, name) {
        Some(s) => return s,
        None => {}
    }
    scope.interfaces.alloc(Interface {
        name: Some(name.to_string()),
        types: Default::default(),
        functions: Default::default(),
        docs: Default::default(),
        package: Some(pid),
    });
    match find_interface(scope, name) {
        Some(s) => return s,
        None => unreachable!(),
    }
}

fn find_interface<'i>(scope: &'i mut Resolve, target: &str) -> Option<(Id<Interface>, &'i mut Interface)> {
    for (w, object) in scope.interfaces.iter_mut() {
        match &object.name {
            Some(s) if s.eq(target) => {
                return Some((w, object));
            }
            _ => {}
        }
    }
    return None;
}

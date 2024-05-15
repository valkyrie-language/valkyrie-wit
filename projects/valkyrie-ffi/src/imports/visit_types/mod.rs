use super::*;
use crate::helpers::TypeContext;
use wit_parser::{Docs, Enum, Flags, Record, Variant};

mod visit_functions;

impl ValkyrieFFI {
    fn export_resource<'a, W: Write>(&self, file: &mut W, ctx: TypeContext<'a>) -> std::fmt::Result {
        match &ctx.def.name {
            Some(wasi_name) => {
                writeln!(file, "#import(\"{}\", \"{}\")", ctx.namespace, wasi_name)?;
                writeln!(file, "class {} {{", wasi_name.to_case(Case::UpperCamel))?;
                for function in ctx.interface.functions.values() {
                    function
                        .write_define(file, FunctionContext { ffi: &self, class_name: wasi_name, namespace: ctx.namespace })?
                }
                writeln!(file, "}}")
            }
            None => panic!("missing name"),
        }
    }
}

impl WriteDefine for TypeDefKind {
    type Context<'a> = TypeContext<'a>;

    fn write_define<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        let wasi_name = match ctx.def.name.as_ref() {
            Some(s) => s.as_str(),
            None => unreachable!(),
        };
        ctx.def.docs.write_define(w, 0)?;
        match self {
            Self::Record(v) => {
                write!(w, "class {} ", wasi_name.to_case(Case::UpperCamel))?;
                v.write_define(w, ctx)?
            }
            Self::Resource => ctx.ffi.export_resource(w, ctx)?,
            Self::Flags(v) => {
                write!(w, "flags {} ", wasi_name.to_case(Case::UpperCamel))?;
                v.write_define(w, ctx)?
            }
            Self::Variant(v) => {
                write!(w, "unite {} ", wasi_name.to_case(Case::UpperCamel))?;
                v.write_define(w, ctx)?
            }
            Self::Enum(v) => v.write_define(w, TypeContext { wasi_name, ..ctx })?,
            Self::Type(v) => {
                write!(w, "alias typus {}: ", wasi_name.to_case(Case::UpperCamel))?;
                v.write_reference(w, ctx.ffi)?;
                w.write_str(" { }\n\n")?;
            }
            Self::List(v) => {
                write!(w, "alias typus {}: Array<", wasi_name.to_case(Case::UpperCamel))?;
                v.write_reference(w, ctx.ffi)?;
                w.write_str("> { }\n\n")?;
            }
            Self::Tuple(v) => {
                write!(w, "alias typus {}: (", wasi_name.to_case(Case::UpperCamel))?;
                for (i, ty) in v.types.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ")?;
                    }
                    ty.write_reference(w, ctx.ffi)?;
                }
                w.write_str(") { }\n\n")?
            }
            Self::Handle(_) => unimplemented!(),
            Self::Option(_) => unimplemented!(),
            Self::Result(_) => unimplemented!(),
            Self::Future(_) => unimplemented!(),
            Self::Stream(_) => unimplemented!(),
            Self::Unknown => unreachable!("UnknownType: {}", wasi_name),
        }
        Ok(())
    }
}
impl WriteDefine for Docs {
    type Context<'a> = usize;

    fn write_define<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        match self.contents.as_ref() {
            Some(s) => {
                for line in s.lines() {
                    for _ in 0..ctx {
                        w.write_str("    ")?
                    }
                    writeln!(w, "⍝? {}", line)?
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}
impl WriteDefine for Record {
    type Context<'a> = TypeContext<'a>;

    fn write_define<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        w.write_str("{\n")?;
        for field in self.fields.iter() {
            field.docs.write_define(w, 1)?;
            write!(w, "    {}: ", field.name.to_case(Case::Snake))?;
            field.ty.write_reference(w, ctx.ffi)?;
            w.write_str(",\n")?
        }
        w.write_str("}\n\n")
    }
}
impl WriteDefine for Enum {
    type Context<'a> = TypeContext<'a>;

    fn write_define<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        // Name is not required
        writeln!(w, "enumerate {} {{", ctx.wasi_name.to_case(Case::UpperCamel))?;
        for variant in self.cases.iter() {
            variant.docs.write_define(w, 1)?;
            writeln!(w, "    #export(\"{}\")", variant.name)?;
            writeln!(w, "    {},", variant.name.to_case(Case::UpperCamel))?;
        }
        w.write_str("}\n\n")
    }
}

impl WriteDefine for Flags {
    type Context<'a> = TypeContext<'a>;

    fn write_define<'a, W: Write>(&self, w: &mut W, _: Self::Context<'a>) -> std::fmt::Result {
        // Name is not required
        w.write_str("{\n")?;
        for field in self.flags.iter() {
            field.docs.write_define(w, 1)?;
            writeln!(w, "    #export(\"{}\")", field.name)?;
            writeln!(w, "    {},", field.name.to_case(Case::Snake))?;
        }
        w.write_str("}\n\n")
    }
}
impl WriteDefine for Variant {
    type Context<'a> = TypeContext<'a>;

    fn write_define<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        w.write_str("{\n")?;
        for variant in self.cases.iter() {
            variant.docs.write_define(w, 1)?;
            writeln!(w, "    #export(\"{}\")", variant.name)?;
            write!(w, "    {}", variant.name.to_case(Case::UpperCamel))?;
            match variant.ty {
                Some(s) => {
                    w.write_str(" {\n        value: ")?;
                    s.write_reference(w, ctx.ffi)?;
                    w.write_str("\n    }")?;
                }
                None => {}
            }
            w.write_str(",\n")?
        }
        w.write_str("}\n\n")
    }
}

impl WriteReference for Results {
    type Context<'a> = &'a ValkyrieFFI;

    fn write_reference<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        match self {
            Self::Named(outputs) => {
                w.write_str(" -> (")?;
                for _ in outputs {
                    unimplemented!()
                }
                w.write_str(")")
            }
            Self::Anon(v) => {
                w.write_str(" -> ")?;
                v.write_reference(w, ctx)
            }
        }
    }
}

impl WriteReference for Type {
    type Context<'a> = &'a ValkyrieFFI;

    fn write_reference<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        match self {
            Self::Bool => w.write_str("bool"),
            Self::U8 => w.write_str("u8"),
            Self::U16 => w.write_str("u16"),
            Self::U32 => w.write_str("u32"),
            Self::U64 => w.write_str("u64"),
            Self::S8 => w.write_str("i8"),
            Self::S16 => w.write_str("i16"),
            Self::S32 => w.write_str("i32"),
            Self::S64 => w.write_str("i64"),
            Self::F32 => w.write_str("f32"),
            Self::F64 => w.write_str("f64"),
            Self::Char => w.write_str("char"),
            Self::String => w.write_str("utf8"),
            Self::Id(v) => v.write_reference(w, ctx),
        }
    }
}

impl WriteReference for TypeId {
    type Context<'a> = &'a ValkyrieFFI;

    fn write_reference<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        match ctx.cache.types.get(*self) {
            Some(s) => s.write_reference(w, ctx),
            None => unreachable!("Unsolved modules"),
        }
    }
}

impl WriteReference for TypeDef {
    type Context<'a> = &'a ValkyrieFFI;

    fn write_reference<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        match &self.kind {
            TypeDefKind::Record(_) => match &self.name {
                Some(s) => w.write_str(&s.to_case(Case::UpperCamel)),
                None => unreachable!(),
            },
            TypeDefKind::Flags(_) => match &self.name {
                Some(s) => w.write_str(&s.to_case(Case::UpperCamel)),
                None => unreachable!(),
            },
            TypeDefKind::Tuple(tuple) => {
                w.write_str("(")?;
                for (i, ty) in tuple.types.iter().enumerate() {
                    if i != 0 {
                        w.write_str(", ")?;
                    }
                    ty.write_reference(w, ctx)?;
                }
                w.write_str(")")
            }
            TypeDefKind::Variant(_) => match &self.name {
                Some(s) => w.write_str(&s.to_case(Case::UpperCamel)),
                None => unreachable!(),
            },
            TypeDefKind::Enum(_) => match &self.name {
                Some(s) => w.write_str(&s.to_case(Case::UpperCamel)),
                None => unreachable!(),
            },
            TypeDefKind::Option(v) => {
                v.write_reference(w, ctx)?;
                w.write_str("?")
            }
            TypeDefKind::Handle(v) => match v {
                Handle::Own(v) => v.write_reference(w, ctx),
                Handle::Borrow(v) => {
                    w.write_str("&")?;
                    v.write_reference(w, ctx)
                }
            },
            TypeDefKind::Resource => match &self.name {
                Some(s) => w.write_str(&s.to_case(Case::UpperCamel)),
                None => unreachable!(),
            },
            TypeDefKind::Result(v) => {
                w.write_str("Result<")?;
                match v.ok {
                    None => w.write_str("()")?,
                    Some(s) => s.write_reference(w, ctx)?,
                }
                w.write_str(", ")?;
                match v.err {
                    None => w.write_str("()")?,
                    Some(s) => s.write_reference(w, ctx)?,
                }
                w.write_str(">")
            }
            TypeDefKind::List(v) => {
                w.write_str("Array<")?;
                v.write_reference(w, ctx)?;
                w.write_str(">")
            }
            TypeDefKind::Future(_) => {
                unimplemented!()
            }
            TypeDefKind::Stream(_) => unimplemented!(),
            TypeDefKind::Type(v) => v.write_reference(w, ctx),
            TypeDefKind::Unknown => unreachable!(),
        }
    }
}

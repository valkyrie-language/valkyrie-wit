use crate::ValkyrieFFI;
use wit_parser::{Interface, TypeDef};

pub(crate) trait WriteDefine {
    type Context<'a>;
    fn write_define<'a, W: std::fmt::Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result;
}
pub(crate) trait WriteReference {
    type Context<'a>;
    fn write_reference<'a, W: std::fmt::Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result;
}
pub(crate) struct FunctionContext<'a> {
    pub ffi: &'a ValkyrieFFI,
    pub class_name: &'a str,
    pub namespace: &'a str,
}
pub(crate) struct TypeContext<'a> {
    pub ffi: &'a ValkyrieFFI,
    pub interface: &'a Interface,
    pub namespace: &'a str,
    pub wasi_name: &'a str,
    pub def: &'a TypeDef,
}

use super::*;

impl ValkyrieFFI {}

impl WriteDefine for Function {
    type Context<'a> = FunctionContext<'a>;

    fn write_define<'a, W: Write>(&self, w: &mut W, ctx: Self::Context<'a>) -> std::fmt::Result {
        match self.kind {
            FunctionKind::Freestanding => {
                if ctx.class_name.is_empty() {
                    ctx.ffi.export_free(self, ctx.namespace, w)?
                }
            }
            FunctionKind::Method(_) => {
                let head = format!("[method]{}.", ctx.class_name);
                if self.name.starts_with(&head) {
                    ctx.ffi.export_method(self, &head, ctx.namespace, w)?
                }
            }
            FunctionKind::Static(_) => {
                let head = format!("[static]{}.", ctx.class_name);
                if self.name.starts_with(&head) {
                    ctx.ffi.export_static(self, &head, ctx.namespace, w)?
                }
            }
            FunctionKind::Constructor(_) => {
                let head = "[constructor]";
                if self.name.starts_with(&head) {
                    ctx.ffi.export_static(self, &head, ctx.namespace, w)?
                }
            }
        }
        Ok(())
    }
}

impl ValkyrieFFI {
    fn export_free<W: Write>(&self, function: &Function, namespace: &str, w: &mut W) -> std::fmt::Result {
        function.docs.write_define(w, 0)?;
        writeln!(w, "↯import(\"{}\", \"{}\")", namespace, function.name)?;
        write!(w, "micro {}(", function.name.to_case(Case::Snake))?;
        for (i, (key, ty)) in function.params.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?
            }
            write!(w, "{}: ", self.parameter_name(key))?;
            ty.write_reference(w, self)?;
        }
        w.write_str(")")?;
        function.results.write_reference(w, self)?;
        w.write_str(" { }\n\n")
    }
    fn export_method<W: Write>(&self, function: &Function, head: &str, namespace: &str, w: &mut W) -> std::fmt::Result {
        function.docs.write_define(w, 1)?;

        writeln!(w, "    ↯import(\"{}\", \"{}\")", namespace, function.name)?;
        write!(w, "    {}(self", function.name.trim_start_matches(&head).to_case(Case::Snake))?;
        for (key, ty) in function.params.iter().skip(1) {
            write!(w, ", {}: ", self.parameter_name(key))?;
            ty.write_reference(w, self)?;
        }
        w.write_str(")")?;
        function.results.write_reference(w, self)?;
        w.write_str(" { }\n\n")
    }
    fn export_static<W: Write>(&self, function: &Function, head: &str, namespace: &str, w: &mut W) -> std::fmt::Result {
        function.docs.write_define(w, 1)?;
        writeln!(w, "    ↯import(\"{}\", \"{}\")", namespace, function.name)?;
        write!(w, "    {}(", function.name.trim_start_matches(&head).to_case(Case::Snake))?;
        for (i, (key, ty)) in function.params.iter().enumerate() {
            if i != 0 {
                w.write_str(", ")?
            }
            write!(w, "{}: ", self.parameter_name(key))?;
            ty.write_reference(w, self)?;
        }
        w.write_str(")")?;
        function.results.write_reference(w, self)?;
        w.write_str(" { }\n\n")
    }
    pub fn parameter_name(&self, input: &str) -> String {
        match input.as_ref() {
            "in" | "when" | "flags" => format!("`{input}`"),
            _ => input.to_case(Case::Snake),
        }
    }
}

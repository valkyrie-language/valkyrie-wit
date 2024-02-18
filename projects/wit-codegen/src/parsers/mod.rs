use crate::ForeignGenerator;
use heck::ToKebabCase;
use id_arena::Id;
use valkyrie_types::{ValkyrieClassType, ValkyrieEnumerate, ValkyrieError};
use wit_parser::{Enum, EnumCase, Interface, TypeDefKind};

impl ForeignGenerator {
    fn load_class(&mut self, m: Id<Interface>, o: &ValkyrieClassType) -> Result<(), ValkyrieError> {
        Ok(())
    }
    fn load_enum(&mut self, m: Id<Interface>, o: &ValkyrieEnumerate) -> Result<(), ValkyrieError> {
        Ok(())
    }
}

use crate::ForeignGenerator;
use id_arena::Id;
use valkyrie_types::{ValkyrieEnumerate, ValkyrieError, ValkyrieStructure};
use wit_parser::Interface;

impl ForeignGenerator {
    fn load_class(&mut self, m: Id<Interface>, o: &ValkyrieStructure) -> Result<(), ValkyrieError> {
        Ok(())
    }
    fn load_enum(&mut self, m: Id<Interface>, o: &ValkyrieEnumerate) -> Result<(), ValkyrieError> {
        Ok(())
    }
}

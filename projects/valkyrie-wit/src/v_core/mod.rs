wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "integer",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        "vit:core/def-integer": RustHost,
    },
});

struct RustHost;
use crate::v_core::exports::vit::core::def_integer::{Guest, Int};
impl Guest for RustHost {
    fn get_position() -> Int {
        todo!()
    }

    fn set_position(pos: Int) {
        todo!()
    }

    fn monsters() -> Vec<Int> {
        todo!()
    }
}
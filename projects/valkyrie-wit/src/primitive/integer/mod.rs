

wit_bindgen::generate!({
    world: "ffi",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        "vit:core/integer": IntegerHost,

    },
});

struct IntegerHost;


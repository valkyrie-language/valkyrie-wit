use valkyrie_ffi::NaturalHost;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    NaturalHost { own: Default::default() };
}

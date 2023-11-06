// Generated by `wit-bindgen` 0.13.2. DO NOT EDIT!
pub mod exports {
    pub mod v {
        pub mod core {

            #[allow(clippy::all)]
            pub mod number {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum Sign {
                    NoSign,
                    Positive,
                    Negative,
                }
                impl ::core::fmt::Debug for Sign {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Sign::NoSign => f.debug_tuple("Sign::NoSign").finish(),
                            Sign::Positive => f.debug_tuple("Sign::Positive").finish(),
                            Sign::Negative => f.debug_tuple("Sign::Negative").finish(),
                        }
                    }
                }

                impl Sign {
                    pub(crate) unsafe fn _lift(val: u8) -> Sign {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }

                        match val {
                            0 => Sign::NoSign,
                            1 => Sign::Positive,
                            2 => Sign::Negative,

                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }

                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Natural {}
                impl ::core::fmt::Debug for Natural {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Natural").finish()
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Integer {
                    pub sign: Sign,
                    pub natural: Natural,
                }
                impl ::core::fmt::Debug for Integer {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Integer").field("sign", &self.sign).field("natural", &self.natural).finish()
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#nat-new-u-64"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_nat_new_u_64(arg0: i64) {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        let result0 = <_GuestImpl as Guest>::nat_new_u_64(arg0 as u64);
                        let Natural {} = result0;
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#nat-add-u-64"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_nat_add_u_64(arg0: i64) {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        let result0 = <_GuestImpl as Guest>::nat_add_u_64(Natural {}, arg0 as u64);
                        let Natural {} = result0;
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#nat-add-u-64-inplace"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_nat_add_u_64_inplace(arg0: i64) {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        <_GuestImpl as Guest>::nat_add_u_64_inplace(Natural {}, arg0 as u64);
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#nat-add-nat"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_nat_add_nat() {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        let result0 = <_GuestImpl as Guest>::nat_add_nat(Natural {}, Natural {});
                        let Natural {} = result0;
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#nat-add-nat-inplace"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_nat_add_nat_inplace() {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        <_GuestImpl as Guest>::nat_add_nat_inplace(Natural {}, Natural {});
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#int-add-u-64"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_int_add_u_64(arg0: i32, arg1: i64) -> i32 {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        let result0 = <_GuestImpl as Guest>::int_add_u_64(
                            Integer { sign: Sign::_lift(arg0 as u8), natural: Natural {} },
                            arg1 as u64,
                        );
                        let Integer { sign: sign1, natural: natural1 } = result0;
                        let Natural {} = natural1;
                        sign1.clone() as i32
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#int-add-nat"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_int_add_nat(arg0: i32) -> i32 {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        let result0 = <_GuestImpl as Guest>::int_add_nat(
                            Integer { sign: Sign::_lift(arg0 as u8), natural: Natural {} },
                            Natural {},
                        );
                        let Integer { sign: sign1, natural: natural1 } = result0;
                        let Natural {} = natural1;
                        sign1.clone() as i32
                    }
                };
                const _: () = {
                    #[doc(hidden)]
                    #[export_name = "v:core/number#int-add-int"]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __export_int_add_int(arg0: i32, arg1: i32) -> i32 {
                        #[allow(unused_imports)]
                        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

                        // Before executing any other code, use this function to run all static
                        // constructors, if they have not yet been run. This is a hack required
                        // to work around wasi-libc ctors calling import functions to initialize
                        // the environment.
                        //
                        // This functionality will be removed once rust 1.69.0 is stable, at which
                        // point wasi-libc will no longer have this behavior.
                        //
                        // See
                        // https://github.com/bytecodealliance/preview2-prototyping/issues/99
                        // for more details.
                        #[cfg(target_arch = "wasm32")]
                        wit_bindgen::rt::run_ctors_once();

                        let result0 = <_GuestImpl as Guest>::int_add_int(
                            Integer { sign: Sign::_lift(arg0 as u8), natural: Natural {} },
                            Integer { sign: Sign::_lift(arg1 as u8), natural: Natural {} },
                        );
                        let Integer { sign: sign1, natural: natural1 } = result0;
                        let Natural {} = natural1;
                        sign1.clone() as i32
                    }
                };
                use super::super::super::super::numberHost as _GuestImpl;
                pub trait Guest {
                    fn nat_new_u_64(n: u64) -> Natural;
                    fn nat_add_u_64(this: Natural, rhs: u64) -> Natural;
                    fn nat_add_u_64_inplace(this: Natural, rhs: u64);
                    fn nat_add_nat(this: Natural, rhs: Natural) -> Natural;
                    fn nat_add_nat_inplace(this: Natural, rhs: Natural);
                    fn int_add_u_64(this: Integer, rhs: u64) -> Integer;
                    fn int_add_nat(this: Integer, rhs: Natural) -> Integer;
                    fn int_add_int(this: Integer, rhs: Integer) -> Integer;
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:core"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 523] = [
    3, 0, 4, 99, 111, 114, 101, 0, 97, 115, 109, 13, 0, 1, 0, 7, 141, 3, 1, 65, 2, 1, 65, 2, 1, 66, 22, 1, 109, 3, 7, 110, 111,
    45, 115, 105, 103, 110, 8, 112, 111, 115, 105, 116, 105, 118, 101, 8, 110, 101, 103, 97, 116, 105, 118, 101, 4, 0, 4, 115,
    105, 103, 110, 3, 0, 0, 1, 114, 0, 4, 0, 7, 110, 97, 116, 117, 114, 97, 108, 3, 0, 2, 1, 114, 2, 4, 115, 105, 103, 110, 1,
    7, 110, 97, 116, 117, 114, 97, 108, 3, 4, 0, 7, 105, 110, 116, 101, 103, 101, 114, 3, 0, 4, 1, 64, 1, 1, 110, 119, 0, 3, 4,
    0, 12, 110, 97, 116, 45, 110, 101, 119, 45, 117, 45, 54, 52, 1, 6, 1, 64, 2, 4, 116, 104, 105, 115, 3, 3, 114, 104, 115,
    119, 0, 3, 4, 0, 12, 110, 97, 116, 45, 97, 100, 100, 45, 117, 45, 54, 52, 1, 7, 1, 64, 2, 4, 116, 104, 105, 115, 3, 3, 114,
    104, 115, 119, 1, 0, 4, 0, 20, 110, 97, 116, 45, 97, 100, 100, 45, 117, 45, 54, 52, 45, 105, 110, 112, 108, 97, 99, 101, 1,
    8, 1, 64, 2, 4, 116, 104, 105, 115, 3, 3, 114, 104, 115, 3, 0, 3, 4, 0, 11, 110, 97, 116, 45, 97, 100, 100, 45, 110, 97,
    116, 1, 9, 1, 64, 2, 4, 116, 104, 105, 115, 3, 3, 114, 104, 115, 3, 1, 0, 4, 0, 19, 110, 97, 116, 45, 97, 100, 100, 45,
    110, 97, 116, 45, 105, 110, 112, 108, 97, 99, 101, 1, 10, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 114, 104, 115, 119, 0, 5,
    4, 0, 12, 105, 110, 116, 45, 97, 100, 100, 45, 117, 45, 54, 52, 1, 11, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3, 114, 104,
    115, 3, 0, 5, 4, 0, 11, 105, 110, 116, 45, 97, 100, 100, 45, 110, 97, 116, 1, 12, 1, 64, 2, 4, 116, 104, 105, 115, 5, 3,
    114, 104, 115, 5, 0, 5, 4, 0, 11, 105, 110, 116, 45, 97, 100, 100, 45, 105, 110, 116, 1, 13, 4, 1, 13, 118, 58, 99, 111,
    114, 101, 47, 110, 117, 109, 98, 101, 114, 5, 0, 4, 1, 11, 118, 58, 99, 111, 114, 101, 47, 99, 111, 114, 101, 4, 0, 11, 16,
    1, 1, 10, 118, 58, 99, 111, 114, 101, 47, 119, 105, 116, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111,
    99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101,
    100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 55, 46, 48, 16, 119,
    105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 51, 46, 50,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}

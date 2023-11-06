use num::BigUint;

/// unsigned integer buffer
#[derive(Debug)]
pub struct NaturalHost {
    /// The owned value of natural
    ///
    /// offset = 0, size = 32
    pub own: BigUint,
}

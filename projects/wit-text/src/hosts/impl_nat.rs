use super::*;
use num_traits::identities::{One, Zero};
impl GuestNatural for NaturalHost {
    fn new(digits: Vec<u32>) -> Self {
        Self { owned: BigUint::new(digits) }
    }
    /// FIXME: Globally unique, requires fixed address
    fn zero() -> OwnNatural {
        Resource::new(Self { owned: BigUint::zero() })
    }
    /// FIXME: Need pinned address
    fn one() -> OwnNatural {
        Resource::new(Self { owned: BigUint::one() })
    }
    // fn new_random(&self, min: OwnNatural, max: OwnNatural, seed: u64) -> OwnNatural {
    //     let mut rng = SmallRng::seed_from_u64(seed).u128();
    //     let unsigned = rng.gen_biguint_range(&min.owned, &max.owned);
    //     Resource::new(Self { owned: unsigned })
    // }
    fn add_u32(&self, rhs: u32) -> OwnNatural {
        Resource::new(Self { owned: self.owned.clone().add(&BigUint::from(rhs)) })
    }

    fn add_u64(&self, rhs: u64) -> OwnNatural {
        Resource::new(Self { owned: self.owned.clone().add(&BigUint::from(rhs)) })
    }

    fn add_nat(&self, rhs: &NaturalHost) -> OwnNatural {
        Resource::new(Self { owned: self.owned.clone().add(&rhs.owned) })
    }
}

use super::*;
use fastrand::Rng;
use num_bigint::RandBigInt;
use rand::{rngs::StdRng, thread_rng, SeedableRng};

impl GuestNatural for NaturalHost {
    fn new(digits: Vec<u32>) -> Self {
        Self { owned: BigUint::new(digits) }
    }

    fn new_random(&self, min: OwnNatural, max: OwnNatural, seed: u64) -> OwnNatural {
        let mut rng = StdRng::seed_from_u64(seed);
        let unsigned = rng.gen_biguint_range(&min.owned, &max.owned);

        let range = rng.u128(0..10);
        Resource::new(Self { owned: unsigned })
    }

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

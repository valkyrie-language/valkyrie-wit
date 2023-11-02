use super::*;

impl GuestNatural for NaturalHost {
    fn new(digits: Vec<u32>) -> Self {
        Self { owned: BigUint::new(digits) }
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

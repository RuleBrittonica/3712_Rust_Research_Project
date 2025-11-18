#[derive(Clone, Copy)]
pub enum Tier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl core::cmp::PartialEq for Tier {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Tier::Bronze,   Tier::Bronze)   => true,
            (Tier::Silver,   Tier::Silver)   => true,
            (Tier::Gold,     Tier::Gold)     => true,
            (Tier::Platinum, Tier::Platinum) => true,
            _ => false,
        }
    }
}

impl core::cmp::Eq for Tier {}

pub struct Customer {
    pub points: u32,
    pub tier: Tier,
}

pub fn update_customer_tier(c: &mut Customer) {
    let new_tier = if c.points < 1_000 {
        Tier::Bronze
    } else if c.points < 5_000 {
        Tier::Silver
    } else if c.points < 10_000 {
        Tier::Gold
    } else {
        Tier::Platinum
    };

    c.tier = new_tier;
}

pub fn run_example() {
    let mut c = Customer {
        points: 4_200,
        tier: Tier::Bronze,
    };

    update_customer_tier(&mut c);

    assert!(c.tier == Tier::Silver);
}

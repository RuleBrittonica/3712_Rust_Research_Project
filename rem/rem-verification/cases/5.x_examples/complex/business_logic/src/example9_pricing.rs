#[derive(Clone, Copy)]
pub enum Region {
    Domestic,
    Eu,
    International,
}

// Manual PartialEq + Eq for Region
impl core::cmp::PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Region::Domestic,      Region::Domestic)      => true,
            (Region::Eu,            Region::Eu)            => true,
            (Region::International, Region::International) => true,
            _ => false,
        }
    }
}

impl core::cmp::Eq for Region {}


pub struct Item {
    pub base_price_cents: u64,
    pub weight_kg: u32,
}

pub fn compute_total_price_cents(
    item: &Item,
    discount_percent: u32,
    region: Region,
) -> u64 {

    // Clamp discount to [0, 100].
    let mut disc = discount_percent;
    if disc > 100 {
        disc = 100;
    }

    // Apply discount: price * (100 - disc) / 100
    let hundred: u64 = 100;
    let base = item.base_price_cents;
    let effective = hundred - (disc as u64);

    let discounted = (base * effective) / hundred;

    // Tax rate by region.
    let tax_rate_percent: u64 = match region {
        Region::Domestic => 10,
        Region::Eu => 20,
        Region::International => 0,
    };

    let tax = (discounted * tax_rate_percent) / hundred;

    // Shipping by weight.
    let shipping: u64 = if item.weight_kg == 0 {
        0
    } else if item.weight_kg < 1 {
        // unreachable because u32, but keep structure simple
        500
    } else if item.weight_kg <= 1 {
        500
    } else if item.weight_kg <= 5 {
        1_000
    } else {
        2_000
    };

    discounted + tax + shipping
}

pub fn run_example() {
    let item = Item {
        base_price_cents: 10_000,
        weight_kg: 2,
    };

    let total = compute_total_price_cents(&item, 10, Region::Domestic);
    // discounted = 9000, tax = 900, shipping = 1000 => total = 10_900
    assert!(total == 10_900);
}

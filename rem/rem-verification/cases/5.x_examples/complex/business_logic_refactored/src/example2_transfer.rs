pub struct Account {
    pub id: u32,
    pub balance_cents: i64,
}

pub fn apply_transfer_effects(src: &mut Account, dst: &mut Account, amount_cents: i64) {
    src.balance_cents -= amount_cents;
    dst.balance_cents += amount_cents;
}

pub fn transfer(src: &mut Account, dst: &mut Account, amount_cents: i64) -> bool {
    if amount_cents <= 0 {
        return false;
    }
    if src.balance_cents < amount_cents {
        return false;
    }

    apply_transfer_effects(src, dst, amount_cents);

    true
}

pub fn run_example() {
    let mut src = Account { id: 1, balance_cents: 10_000 };
    let mut dst = Account { id: 2, balance_cents: 5_000 };

    let ok = transfer(&mut src, &mut dst, 3_000);
    assert!(ok);
    assert!(src.balance_cents == 7_000);
    assert!(dst.balance_cents == 8_000);
}

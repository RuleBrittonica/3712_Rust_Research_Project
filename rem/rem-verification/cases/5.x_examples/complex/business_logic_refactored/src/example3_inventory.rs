pub struct Item {
    pub id: i32,
    pub stock: i32,
    pub reserved: i32,
}

pub fn reconcile_reserved(item: &mut Item) {
    if item.reserved > item.stock {
        item.reserved = item.stock;
    }
}

pub fn apply_stock_change(item: &mut Item, delta: i32) -> bool {
    let new_stock = item.stock + delta;
    let mut ok = true;

    if new_stock < 0 {
        item.stock = 0;
        ok = false;
    } else {
        item.stock = new_stock;
    }

    reconcile_reserved(item);

    ok
}

pub fn run_example() {
    let mut item = Item {
        id: 10,
        stock: 5,
        reserved: 2,
    };

    let ok1 = apply_stock_change(&mut item, -3);
    assert!(ok1);
    assert!(item.stock == 2);
    assert!(item.reserved == 2);

    let ok2 = apply_stock_change(&mut item, -10);
    assert!(!ok2);
    assert!(item.stock == 0);
    assert!(item.reserved == 0);
}

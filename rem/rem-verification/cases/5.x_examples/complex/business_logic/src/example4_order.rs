#[derive(Clone, Copy)]
pub enum OrderState {
    Created,
    Paid,
    Shipped,
    Cancelled,
}

// Manual PartialEq + Eq for OrderState
impl core::cmp::PartialEq for OrderState {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (OrderState::Created,   OrderState::Created)   => true,
            (OrderState::Paid,      OrderState::Paid)      => true,
            (OrderState::Shipped,   OrderState::Shipped)   => true,
            (OrderState::Cancelled, OrderState::Cancelled) => true,
            _ => false,
        }
    }
}

impl core::cmp::Eq for OrderState {}

#[derive(Clone, Copy)]
pub enum Event {
    Pay,
    Ship,
    Cancel,
}

impl core::cmp::PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Event::Pay,    Event::Pay)    => true,
            (Event::Ship,   Event::Ship)   => true,
            (Event::Cancel, Event::Cancel) => true,
            _ => false,
        }
    }
}

impl core::cmp::Eq for Event {}

pub fn apply_event(state: &mut OrderState, event: Event) -> bool {
    // Original, non-extracted state transition logic.
    let current = *state;
    let mut changed = false;

    match current {
        OrderState::Created => {
            match event {
                Event::Pay => {
                    *state = OrderState::Paid;
                    changed = true;
                }
                Event::Cancel => {
                    *state = OrderState::Cancelled;
                    changed = true;
                }
                Event::Ship => {
                    // illegal
                }
            }
        }
        OrderState::Paid => {
            match event {
                Event::Ship => {
                    *state = OrderState::Shipped;
                    changed = true;
                }
                Event::Cancel => {
                    *state = OrderState::Cancelled;
                    changed = true;
                }
                Event::Pay => {
                    // already paid
                }
            }
        }
        OrderState::Shipped => {
            // No further transitions allowed.
        }
        OrderState::Cancelled => {
            // No further transitions allowed.
        }
    }

    changed
}

pub fn run_example() {
    let mut state = OrderState::Created;
    let c1 = apply_event(&mut state, Event::Pay);
    assert!(c1);
    assert!(state == OrderState::Paid);

    let c2 = apply_event(&mut state, Event::Ship);
    assert!(c2);
    assert!(state == OrderState::Shipped);

    let c3 = apply_event(&mut state, Event::Cancel);
    assert!(!c3);
    assert!(state == OrderState::Shipped);
}

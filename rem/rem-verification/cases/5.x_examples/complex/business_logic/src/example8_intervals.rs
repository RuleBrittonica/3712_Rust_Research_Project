#[derive(Clone, Copy)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

// Manual PartialEq + Eq for Interval
impl core::cmp::PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        // Both fields are integers → safe to compare directly
        self.start == other.start && self.end == other.end
    }
}

impl core::cmp::Eq for Interval {}

#[derive(Clone, Copy)]
pub enum MergeResultKind {
    Disjoint,
    Merged,
}

// Manual PartialEq + Eq for MergeResultKind
impl core::cmp::PartialEq for MergeResultKind {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (MergeResultKind::Disjoint, MergeResultKind::Disjoint) => true,
            (MergeResultKind::Merged,   MergeResultKind::Merged)   => true,
            _ => false,
        }
    }
}

impl core::cmp::Eq for MergeResultKind {}

pub struct MergeResult {
    pub kind: MergeResultKind,
    pub first: Interval,
    pub second: Interval,
}

pub fn merge_intervals(a: Interval, b: Interval) -> MergeResult {
    // Ensure left.start <= right.start.
    let mut left = a;
    let mut right = b;
    if left.start > right.start {
        let tmp = left;
        left = right;
        right = tmp;
    }

    // If they are disjoint and not touching: [..left..]  [..right..]
    // Otherwise, return a single merged interval in `first`.
    let mut kind = MergeResultKind::Merged;
    let mut merged = Interval {
        start: left.start,
        end: left.end,
    };
    let mut other = Interval {
        start: right.start,
        end: right.end,
    };

    // Consider them overlapping or touching if left.end + 1 >= right.start.
    if left.end + 1 < right.start {
        kind = MergeResultKind::Disjoint;
        // In disjoint case, keep them as left, right.
        merged = left;
        other = right;
    } else {
        // Merged case.
        if right.end > merged.end {
            merged.end = right.end;
        }
        // `other` is unused in this case but kept for a uniform struct.
        other = merged;
    }

    MergeResult {
        kind,
        first: merged,
        second: other,
    }
}

pub fn run_example() {
    let a = Interval { start: 0, end: 5 };
    let b = Interval { start: 6, end: 10 };
    let r1 = merge_intervals(a, b);

    assert!(r1.kind == MergeResultKind::Merged);
    assert!(r1.first.start == 0 && r1.first.end == 10);

    let c = Interval { start: 0, end: 3 };
    let d = Interval { start: 10, end: 12 };
    let r2 = merge_intervals(c, d);
    assert!(r2.kind == MergeResultKind::Disjoint);
}

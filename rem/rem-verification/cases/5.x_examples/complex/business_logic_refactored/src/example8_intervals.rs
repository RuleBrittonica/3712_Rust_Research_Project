#[derive(Clone, Copy)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl core::cmp::PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}
impl core::cmp::Eq for Interval {}

#[derive(Clone, Copy)]
pub enum MergeResultKind {
    Disjoint,
    Merged,
}

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

pub fn canonicalise_intervals(a: Interval, b: Interval) -> (Interval, Interval) {
    if a.start <= b.start {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn merge_intervals(a: Interval, b: Interval) -> MergeResult {
    let (mut left, mut right) = canonicalise_intervals(a, b);

    let mut kind = MergeResultKind::Merged;
    let mut merged = Interval {
        start: left.start,
        end: left.end,
    };
    let mut other = Interval {
        start: right.start,
        end: right.end,
    };

    if left.end + 1 < right.start {
        kind = MergeResultKind::Disjoint;
        merged = left;
        other = right;
    } else {
        if right.end > merged.end {
            merged.end = right.end;
        }
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

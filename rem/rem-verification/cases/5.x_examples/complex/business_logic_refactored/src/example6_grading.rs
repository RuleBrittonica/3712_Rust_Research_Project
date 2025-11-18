pub struct Components {
    pub exam: u32,
    pub assignments: u32,
    pub participation: u32,
}

pub fn compute_weighted_mark(c: &Components) -> u32 {
    let exam_part = 4 * c.exam;
    let assign_part = 4 * c.assignments;
    let part_part = 2 * c.participation;
    let sum = exam_part + assign_part + part_part;

    sum / 10
}

pub fn compute_final_mark(c: &Components) -> u32 {
    if c.exam < 40 {
        return 49;
    }

    compute_weighted_mark(c)
}

pub fn run_example() {
    let c1 = Components { exam: 35, assignments: 90, participation: 100 };
    let g1 = compute_final_mark(&c1);
    assert!(g1 == 49);

    let c2 = Components { exam: 80, assignments: 70, participation: 50 };
    let g2 = compute_final_mark(&c2);
    // 0.4*80 + 0.4*70 + 0.2*50 = 32 + 28 + 10 = 70
    assert!(g2 == 70);
}

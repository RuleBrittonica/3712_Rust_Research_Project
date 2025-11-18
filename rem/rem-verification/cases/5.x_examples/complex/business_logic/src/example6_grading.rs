pub struct Components {
    pub exam: u32,         // 0..100
    pub assignments: u32,  // 0..100
    pub participation: u32 // 0..100
}

pub fn compute_final_mark(c: &Components) -> u32 {
    // If exam < 40, cap at 49.
    if c.exam < 40 {
        return 49;
    }

    // Weighted integer grade:
    // 0.4 * exam + 0.4 * assignments + 0.2 * participation
    // Use scaled integer arithmetic.
    let exam_part = 4 * c.exam;
    let assign_part = 4 * c.assignments;
    let part_part = 2 * c.participation;
    let sum = exam_part + assign_part + part_part;

    sum / 10
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

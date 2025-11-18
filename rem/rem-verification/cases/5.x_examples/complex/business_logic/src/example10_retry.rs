pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u32,
    pub backoff_factor: u32,
}

pub fn compute_total_backoff(cfg: &RetryConfig) -> u32 {
    if cfg.max_attempts == 0 {
        return 0;
    }

    let mut attempts: u32 = 0;
    let mut delay: u32 = cfg.initial_delay_ms;
    let mut total: u32 = 0;

    while attempts < cfg.max_attempts {
        total = total.saturating_add(delay);
        delay = delay.saturating_mul(cfg.backoff_factor);
        attempts += 1;
    }

    total
}

pub fn run_example() {
    let cfg = RetryConfig {
        max_attempts: 3,
        initial_delay_ms: 100,
        backoff_factor: 2,
    };

    let total = compute_total_backoff(&cfg);
    // delays: 100 + 200 + 400 = 700
    assert!(total == 700);
}

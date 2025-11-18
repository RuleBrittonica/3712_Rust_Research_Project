mod example1_loyalty;
mod example2_transfer;
mod example3_inventory;
mod example4_order;
mod example5_config;
mod example6_grading;
mod example7_moving_average;
mod example8_intervals;
mod example9_pricing;
// mod example10_retry;

fn main() {
    crate::example1_loyalty::run_example();
    crate::example2_transfer::run_example();
    crate::example3_inventory::run_example();
    crate::example4_order::run_example();
    crate::example5_config::run_example();
    crate::example6_grading::run_example();
    crate::example7_moving_average::run_example();
    crate::example8_intervals::run_example();
    crate::example9_pricing::run_example();
    // crate::example10_retry::run_example();
}

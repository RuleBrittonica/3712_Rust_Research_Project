pub mod common;
pub mod repair_lifetime_loosest_bound_first;
pub mod repair_lifetime_simple;
pub mod repair_lifetime_tightest_bound_first;
pub mod repair_rustfix;

mod exports;
pub use exports::call_all_repairers as repairer_main;
pub use exports::{RepairerInput, RepairReturn};
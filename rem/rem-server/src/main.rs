mod stdio;
mod init;
mod utils;

mod extract;
mod repairer;
mod verification;

use stdio::run_stdio_server;


fn main() {
    let _ = run_stdio_server();
}
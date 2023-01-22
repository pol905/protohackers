use clap::{Parser};
use protohackers_tcp_helper::cli_helper::Args;

// Points to remember:
// 1. When client first connects it doesn't have a name, send a welcome message asking
// them what to be called. To which the client will reply with a name.
// 2. all messages will be ascii
// 3. name validation: min. 1 char, max. 16 chars, uppercase, lowercase, and digits.
//
fn main() {
    let _args = Args::parse();

}

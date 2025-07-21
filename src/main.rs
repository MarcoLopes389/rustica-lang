mod frontend;
mod runtime;
use std::env::args;

use runtime::{repl::Repl, runner::Runner};

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() > 1 {
        Runner::run(args[1].as_str())
    } else {
        Repl::run();
    }
}

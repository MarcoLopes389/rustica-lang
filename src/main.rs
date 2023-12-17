mod frontend;
mod runtime;
use std::env::args;

use runtime::{repl::Repl, runner::Runner};

fn main() {
    if args().len() > 1 {
        match args().next() {
            Some(arg) => Runner::run(arg.as_str()),
            None => print!("Not recognized argument"),
        }
    } else {
        Repl::run();
    }
}

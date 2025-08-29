mod initialize;
use std::env::args;

fn main() {
    let command_args : Vec<String> = args().collect();

    if command_args[1] == "init"{
        initialize::init();
    }
}

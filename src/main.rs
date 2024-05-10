use std::{
    env::{self, args},
    fs,
    process::exit,
};

use racing_sorts::{racer::test, screen::ScreenManager, sorts::KNOWN_SORTS};

fn usage(message: &str) -> ! {
    println!("{}", message);
    println!(
        "usage: racing_sorts -f <filename> <sortAlgorithm> [<sortAlgorithm> ...]
Where <sortAlgorithm> is one of:"
    );
    for sort in KNOWN_SORTS {
        println!("\t{}", sort);
    }
    exit(1);
}

fn main() {
    test();
}

use std::{
    env::{self, args},
    fs,
    process::exit,
};

use racing_sorts::{racer::sort_manager, screen::ScreenManager, sorts::KNOWN_SORTS};

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
    let mut args = args();
    let mut sorts = Vec::new();
    args.next().unwrap();
    // eats -f, type it right!
    args.next()
        .unwrap_or_else(|| usage("please type the -f, I'm begging you"));
    let filename = args
        .next()
        .unwrap_or_else(|| usage("No filename passed in"));
    let data = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        })
        .into_bytes();
    let sort1 = args
        .next()
        .unwrap_or_else(|| usage("Atleast one sort needs to be provided"));
    sorts.push(sort1);
    for sort in args {
        sorts.push(sort);
    }
    let res = sort_manager(sorts, data);
    if let Err(message) = res {
        usage(message);
    }
}

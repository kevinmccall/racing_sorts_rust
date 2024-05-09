use std::{
    env,
    io::{stdout, Write},
    thread,
    time::Duration,
};

use racing_sorts::{racer::sort_manager, screen::ScreenManager};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let my_data = "On your first day at the new job, squash every commit from the repo into a single commit with message 'Legacy code' and force-push to master".bytes().collect();
    // let test_data = "abcdefghijklmnopqrstuvwxyz".bytes().collect();
    sort_manager(my_data);
}

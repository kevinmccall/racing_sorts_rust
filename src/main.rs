use std::{
    env,
    io::{stdout, Write},
    thread,
    time::Duration,
};

use racing_sorts::{racer::sort_manager, screen::ScreenManager};

fn main() {
    let my_data = "On your first day at the new job, squash every commit from the repo into a single commit with message 'Legacy code' and force-push to master".bytes().collect();
    // let my_data = "hgfedcba".bytes().collect();
    sort_manager(my_data);
}

#[allow(dead_code, unused_imports)]
use std::io;

use aoc{{year}}::*;

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {

        // Day invocations
        _ => {}
    };

    Ok(())
}
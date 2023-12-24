use std::time::Instant;

use day_22::part2::process;

fn main() {
    let file = include_str!("../../input.txt");
    let now = Instant::now();
    let result = process(file);
    println!("{} : {}s", result, now.elapsed().as_secs());
}
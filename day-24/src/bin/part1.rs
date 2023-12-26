use day_24::part1::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file, 200000000000000.0, 400000000000000.0);
    println!("{}", result);
}
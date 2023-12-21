use day_21::part1::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file, 64);
    println!("{}", result);
}
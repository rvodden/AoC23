use day_24::part2::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file, 0.0, 0.0);
    println!("{}", result);
}
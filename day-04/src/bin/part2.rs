use day_04::part2::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);
}
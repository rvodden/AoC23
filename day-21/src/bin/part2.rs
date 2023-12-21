use day_21::part2::process;

fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file, 26501365);
    println!("{}", result);
}
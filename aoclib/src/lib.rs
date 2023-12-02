use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file_line_by_line(filepath: &str) -> Vec<String> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => panic!("Panic! {}", e),
    };
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

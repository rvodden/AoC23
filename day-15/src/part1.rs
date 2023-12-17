pub fn process<'a>(
    input: &'a str,
) -> u32 {
    input.split(",").map(|cs| cs.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256 )).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let lines = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = process(lines);
        assert_eq!(result, 1320);
    }
}

mod part;
pub use part::*;

pub fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines();
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let digits = line.split_ascii_whitespace().collect::<Vec<_>>();
        left.push(digits[0].parse().unwrap());
        right.push(digits[1].parse().unwrap());
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let (left, right) = parse(SAMPLE);
        assert_eq!(left.len(), 6);
        assert_eq!(right.len(), 6);
    }
}

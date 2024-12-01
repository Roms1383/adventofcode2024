mod part;
pub use part::*;

pub fn parse_v1(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines();
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let digits = line.split_ascii_whitespace().collect::<Vec<_>>();
        left.push(digits[0].parse().expect("usize"));
        right.push(digits[1].parse().expect("usize"));
    }
    (left, right)
}

pub fn parse_v2(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines();
    let len: usize = lines.clone().count();
    let mut left = Vec::with_capacity(len);
    let mut right = Vec::with_capacity(len);
    let mut digits: Vec<&str>;

    for line in lines {
        digits = line.split_ascii_whitespace().collect();
        left.push(digits[0].parse().expect("usize"));
        right.push(digits[1].parse().expect("usize"));
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let (left, right) = parse_v1(SAMPLE);
        assert_eq!(left.len(), 6);
        assert_eq!(right.len(), 6);
    }
}

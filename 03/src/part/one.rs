use super::Mul;

pub fn parse(input: &str) -> Vec<Mul> {
    let mut out = vec![];
    let mut mul = 0;
    let mut left = String::new();
    let mut right = String::new();
    for char in input.chars() {
        if !char.is_ascii_digit() && !['m', 'u', 'l', '(', ',', ')'].contains(&char) {
            mul = 0;
            left.clear();
            right.clear();
            continue;
        }
        match (char, mul) {
            ('m', 0) => {
                mul = 1;
            }
            ('u', 1) | ('l', 2) | ('(', 3) => {
                mul += 1;
            }
            (char, 4) if char.is_ascii_digit() => {
                left.push(char);
            }
            (',', 4) => {
                mul += 1;
                continue;
            }
            (char, 5) if char.is_ascii_digit() => {
                right.push(char);
            }
            (')', 5) => {
                out.push(Mul(left.parse().unwrap(), right.parse().unwrap()));
                mul = 0;
                left.clear();
                right.clear();
                continue;
            }
            _ => continue,
        }
    }
    out
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    use super::parse;
    use crate::Mul;

    #[test]
    fn can_parse() {
        let muls = parse(SAMPLE);
        assert_eq!(muls.len(), 4);
    }

    #[test]
    fn can_multiply() {
        let muls = parse(SAMPLE);
        assert_eq!(muls.iter().map(|Mul(a, b)| a * b).sum::<usize>(), 161);
    }
}

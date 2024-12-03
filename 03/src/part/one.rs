pub struct Mul(pub usize, pub usize);

pub fn parse(input: &str) -> Vec<Mul> {
    let mut out = vec![];
    let mut mul = 0;
    let mut operator = String::new();
    let mut operand = String::new();
    for char in input.chars().into_iter() {
        if !char.is_digit(10) && !['m', 'u', 'l', '(', ',', ')'].contains(&char) {
            mul = 0;
            operator.clear();
            operand.clear();
            continue;
        }
        match (char, mul) {
            ('m', 0) => {
                mul = 1;
            }
            ('u', 1) | ('l', 2) | ('(', 3) => {
                mul += 1;
            }
            (char, 4) if char.is_digit(10) => {
                operator.push(char);
            }
            (',', 4) => {
                mul += 1;
                continue;
            }
            (char, 5) if char.is_digit(10) => {
                operand.push(char);
            }
            (')', 5) => {
                out.push(Mul(operator.parse().unwrap(), operand.parse().unwrap()));
                mul = 0;
                operator.clear();
                operand.clear();
                continue;
            }
            _ => continue,
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::{part::SAMPLE, Mul};

    use super::parse;

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

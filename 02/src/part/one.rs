pub struct Report(Vec<usize>);
pub trait Safe {
    fn safe(&self) -> bool;
}
impl Safe for Report {
    fn safe(&self) -> bool {
        let mut asc = true; // just to satisfy compiler
        let mut diff;
        let mut iter = self.0.windows(2).enumerate();
        while let Some((idx, &[current, next])) = iter.next() {
            if idx == 0 {
                if current == next {
                    return false;
                }
                asc = current < next;
            }
            if (asc && current > next) || (!asc && current < next) {
                return false;
            }
            diff = current.abs_diff(next);
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
        true
    }
}

pub fn parse(input: &str) -> Vec<Report> {
    let lines = input.lines();
    let mut reports = vec![];

    for line in lines {
        let report = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        reports.push(Report(report));
    }
    reports
}

#[cfg(test)]
mod tests {
    use crate::{part::SEED, Safe};

    use super::parse;

    #[test]
    fn can_parse() {
        let input = parse(SEED);
        assert_eq!(input.len(), 6);
    }

    #[test]
    fn can_check_safety() {
        let input = parse(SEED);
        assert!(input[0].safe());
        assert!(!input[1].safe());
        assert!(!input[2].safe());
        assert!(!input[3].safe());
        assert!(!input[4].safe());
        assert!(input[5].safe());
    }
}

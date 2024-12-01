fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let mut left = vec![];
    let mut right = vec![];

    while let Some(line) = lines.next() {
        let digits = line.split_ascii_whitespace().collect::<Vec<_>>();
        left.push(digits[0].parse().unwrap());
        right.push(digits[1].parse().unwrap());
    }
    (left, right)
}

fn distances(mut left: Vec<usize>, mut right: Vec<usize>) -> Vec<usize> {
    let mut distances = Vec::with_capacity(left.len());
    left.sort();
    right.sort();
    for (left, right) in left.iter().zip(right.iter()) {
        distances.push(left.abs_diff(*right));
    }
    distances
}

fn similarities(left: &[usize], right: &[usize]) -> Vec<usize> {
    let mut similarities = Vec::with_capacity(left.len());
    for left in left.iter() {
        similarities.push(left * right.iter().filter(|right| **right == *left).count());
    }
    similarities
}

fn main() {
    let input = include_str!("../part-I.txt");
    let (left, right) = parse(input);
    let distances = distances(left, right);
    println!("part I: {}", distances.iter().sum::<usize>());

    let input = include_str!("../part-II.txt");
    let (left, right) = parse(input);
    let similarities = similarities(&left, &right);
    println!("part II: {}", similarities.iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_ONE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    const PART_TWO: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn can_parse() {
        let (left, right) = parse(PART_ONE);
        assert_eq!(left.len(), 6);
        assert_eq!(right.len(), 6);
    }

    #[test]
    fn can_get_distances() {
        let (left, right) = parse(PART_ONE);
        let distances = distances(left, right);
        assert_eq!(distances[0], 2);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], 0);
        assert_eq!(distances[3], 1);
        assert_eq!(distances[4], 2);
        assert_eq!(distances[5], 5);
    }

    #[test]
    fn can_get_similarities() {
        let (left, right) = parse(PART_TWO);
        let similarities = similarities(&left, &right);
        assert_eq!(similarities[0], 9);
        assert_eq!(similarities[1], 4);
        assert_eq!(similarities[2], 0);
        assert_eq!(similarities[3], 0);
        assert_eq!(similarities[4], 9);
        assert_eq!(similarities[5], 9);
    }
}

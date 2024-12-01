pub fn distances(mut left: Vec<usize>, mut right: Vec<usize>) -> Vec<usize> {
    let mut distances = Vec::with_capacity(left.len());
    left.sort();
    right.sort();
    for (left, right) in left.iter().zip(right.iter()) {
        distances.push(left.abs_diff(*right));
    }
    distances
}

#[cfg(test)]
mod tests {
    use crate::{parse_v1, SAMPLE};

    use super::*;

    #[test]
    fn can_get_distances() {
        let (left, right) = parse_v1(SAMPLE);
        let distances = distances(left, right);
        assert_eq!(distances[0], 2);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], 0);
        assert_eq!(distances[3], 1);
        assert_eq!(distances[4], 2);
        assert_eq!(distances[5], 5);
    }
}

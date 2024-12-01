use std::collections::HashMap;

pub fn similarities_v1(left: &[usize], right: &[usize]) -> Vec<usize> {
    let mut similarities = Vec::with_capacity(left.len());
    for left in left.iter() {
        similarities.push(left * right.iter().filter(|right| **right == *left).count());
    }
    similarities
}

pub fn similarities_v2(left: &[usize], right: &[usize]) -> Vec<usize> {
    struct Similarity {
        pub similarity: usize,
        pub count: usize,
    }
    let mut similarities: HashMap<usize, Similarity> = HashMap::with_capacity(left.len());
    for left in left.iter() {
        if let Some(similarity) = similarities.get_mut(left) {
            similarity.count += 1;
        } else {
            let similarity = left * right.iter().filter(|right| **right == *left).count();
            similarities.insert(
                *left,
                Similarity {
                    similarity,
                    count: 1,
                },
            );
        }
    }
    similarities
        .values()
        .filter(|Similarity { similarity, .. }| similarity > &0)
        .map(|Similarity { similarity, count }| similarity * count)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse, SAMPLE};

    use super::*;

    #[test]
    fn can_get_similarities() {
        let (left, right) = parse(SAMPLE);
        let similarities = similarities_v1(&left, &right);
        assert_eq!(similarities[0], 9);
        assert_eq!(similarities[1], 4);
        assert_eq!(similarities[2], 0);
        assert_eq!(similarities[3], 0);
        assert_eq!(similarities[4], 9);
        assert_eq!(similarities[5], 9);

        let (left, right) = parse(SAMPLE);
        let similarities = similarities_v2(&left, &right);
        assert_eq!(similarities[0], 27);
        assert_eq!(similarities[1], 4);
    }
}

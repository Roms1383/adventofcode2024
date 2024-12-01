use day_01::*;

fn main() {
    let input = include_str!("../part-I.txt");
    let (left, right) = parse_v1(input);
    let distances = distances(left, right);
    println!("part I: {}", distances.iter().sum::<usize>());

    let input = include_str!("../part-II.txt");
    let (left, right) = parse_v1(input);
    let similarities = similarities_v1(&left, &right);
    println!("part II: {}", similarities.iter().sum::<usize>());
}

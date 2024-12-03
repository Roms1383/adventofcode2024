use day_03::*;

fn main() {
    let input = include_str!("../part-I.txt");
    let multiplications = parse(input);
    let sum: usize = multiplications.iter().map(|Mul(a, b)| a * b).sum();
    println!("part I: {}", sum);
}

use day_02::*;

fn main() {
    let input = include_str!("../part-I.txt");
    let reports = parse(input);
    let safe = reports.iter().filter(|x| x.safe()).count();
    println!("part I: {}", safe);
}

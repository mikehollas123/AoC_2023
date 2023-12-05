mod part_1;
mod part_2;

fn main() {
    let result1 = part_1::process(include_str!("input.txt"));
    print!("day-01 part1: {}\n",result1);

    let result2 = part_2::process(include_str!("input.txt"));
    print!("day-01 part2: {}\n",result2);
}

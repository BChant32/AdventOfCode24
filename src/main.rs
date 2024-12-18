use std::fs;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input_content: String = fs::read_to_string("./src/Inputs/day5_input.txt")
        .expect("Should be able to read input file");
    let result: i32 = day5::part2(input_content.as_ref());
    print!("Result: {result}");
}

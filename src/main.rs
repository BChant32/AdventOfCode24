use std::fs;
mod day1;
mod day2;

fn main() {
    let input_content: String = fs::read_to_string("./src/Inputs/day2_input.txt")
        .expect("Should be able to read input file");
    let result: i32 = day2::part2(input_content.as_ref());
    print!("Result: {result}");
}

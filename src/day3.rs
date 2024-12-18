use regex::Regex;

fn scan_line(line: &str) -> i32 {
    let mut line_sum: i32 = 0;
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex.captures_iter(line).for_each(|cap| {
        line_sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    });
    line_sum
}

pub(crate) fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        sum += scan_line(line);
    }
    sum
}

fn scan_line_do(line: &str, enabled: &mut bool) -> i32 {
    let mut line_sum: i32 = 0;
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    regex.captures_iter(line).for_each(|cap| {
        if cap[0].eq_ignore_ascii_case("do()") {
            *enabled = true;
        } else if cap[0].eq_ignore_ascii_case("don't()") {
            *enabled = false;
        } else if *enabled {
            line_sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }
    });
    line_sum
}

pub(crate) fn part2(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut enabled: bool = true;
    for line in input.lines() {
        sum += scan_line_do(line, &mut enabled);
    }
    sum
}
use std::ops::BitXor;

fn isSafe(levels: &Vec<i32>) -> bool {
    let increasing: bool = levels[1] - levels[0] > 0;
    for i in 1..levels.len() {
        let dif = levels[i] - levels[i -1];
        if dif == 0 { return false }
        else if dif.abs() > 3 { return false }
        else if (dif > 0).bitxor(increasing) { return false }
    }
    true
}

fn couldBeSafe(levels: &Vec<i32>) -> bool {
    if isSafe(levels) { return true }
    for i in 0..levels.len() {
        let subset: Vec<i32> = levels[..i].iter().copied()
            .chain(levels[(i + 1)..].iter().copied())
            .collect();
        if isSafe(&subset) { return true }
    }
    false
}

pub(crate) fn part1(input: &str) -> i32 {
    let mut count: i32 = 0;
    for line in input.lines() {
        let splits: Vec<&str> = line.split(' ').collect();
        let levels: Vec<i32> = splits.iter().map(|&s| s.parse::<i32>().unwrap()).collect();
        if isSafe(&levels) { count += 1; }
    }
    count
}

pub(crate) fn part2(input: &str) -> i32 {
    let mut count: i32 = 0;
    for line in input.lines() {
        let splits: Vec<&str> = line.split(' ').collect();
        let levels: Vec<i32> = splits.iter().map(|&s| s.parse::<i32>().unwrap()).collect();
        if couldBeSafe(&levels) { count += 1; }
    }
    count
}
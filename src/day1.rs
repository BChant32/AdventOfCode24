pub(crate) fn _part1(input_content: &str) -> i32 {
    let mut lefties: Vec<i32> = Vec::<i32>::new();
    let mut righties: Vec<i32> = Vec::<i32>::new();
    input_content.lines().for_each(|line| {
        let break_index: usize = line.find("   ").unwrap();
        lefties.push(line[..break_index].parse::<i32>().unwrap());
        righties.push(line[break_index + 3..].parse::<i32>().unwrap());
    });
    lefties.sort();
    righties.sort();
    let mut sum: i32 = 0;
    for i in 0..lefties.len() {
        sum += (lefties[i] - righties[i]).abs();
    }
    sum
}

pub(crate) fn _part2(input_content: &str) -> i32 {
    let mut lefties: Vec<i32> = Vec::<i32>::new();
    let mut righties: Vec<i32> = Vec::<i32>::new();
    input_content.lines().for_each(|line| {
        let break_index: usize = line.find("   ").unwrap();
        lefties.push(line[..break_index].parse::<i32>().unwrap());
        righties.push(line[break_index + 3..].parse::<i32>().unwrap());
    });
    lefties.sort();
    righties.sort();
    let mut sum: i32 = 0;
    for i in 0..lefties.len() {
        sum += (lefties[i] * righties.iter().filter(|&r| *r == lefties[i]).count() as i32).abs();
    }
    sum
}
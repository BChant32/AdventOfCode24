fn valid_eqn(total: u64, operands: Vec<u64>) -> bool {
    if operands.len() == 1 { return operands[0] == total; }
    if operands[0] > total { return false; }

    let mut with_add = vec![operands[0] + operands[1]];
    with_add.extend_from_slice(operands[2..].as_ref());
    let mut with_mult = vec![operands[0] * operands[1]];
    with_mult.extend_from_slice(operands[2..].as_ref());

    valid_eqn(total, with_add) || valid_eqn(total, with_mult)
}

pub(crate) fn _part1(input: &str) -> u64 {
    let _trial: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let mut sum: u64 = 0;
    for line in input.lines() {
        let (total_str, rest_str) = line.split_once(':').unwrap();
        let total = total_str.parse::<u64>().unwrap();
        let operands: Vec<u64> = rest_str.trim().split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
        if valid_eqn(total, operands) {
            sum += total;
        }
    }
    sum
}

fn valid_eqn2(total: u64, operands: Vec<u64>) -> bool {
    if operands.len() == 1 { return operands[0] == total; }
    if operands[0] > total { return false; }

    let mut with_add = vec![operands[0] + operands[1]];
    with_add.extend_from_slice(operands[2..].as_ref());
    let mut with_mult = vec![operands[0] * operands[1]];
    with_mult.extend_from_slice(operands[2..].as_ref());
    let mut with_concat = vec![(operands[0].to_string() + operands[1].to_string().as_str()).parse::<u64>().unwrap()];
    with_concat.extend_from_slice(operands[2..].as_ref());

    valid_eqn2(total, with_add) || valid_eqn2(total, with_mult) || valid_eqn2(total, with_concat)
}

pub(crate) fn part2(input: &str) -> u64 {
    let _trial: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let mut sum: u64 = 0;
    for line in input.lines() {
        let (total_str, rest_str) = line.split_once(':').unwrap();
        let total = total_str.parse::<u64>().unwrap();
        let operands: Vec<u64> = rest_str.trim().split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
        if valid_eqn2(total, operands) {
            sum += total;
        }
    }
    sum
}
fn right_order(rules: &Vec<(u8, u8)>, pages_str: &str) -> u8 {
    let pages: Vec<u8> = pages_str.split(',').map(|i| i.parse::<u8>().unwrap()).collect();
    for (a, b) in rules {
        if pages.contains(a) && pages.contains(b) {
            if pages.iter().position(|x| x == a) > pages.iter().position(|x| x == b)
                {return 0;}
        }
    }
    pages[(pages.len() - 1) / 2]
}

pub(crate) fn part1(input: &str) -> i32 {
    let mut rules: Vec<(u8, u8)> = Vec::new();
    let mut is_header = true;
    let mut sum: i32 = 0;
    input.lines().for_each(|line| {
        if line.is_empty() {
            is_header = false;
        } else if is_header {
            let a = line[..2].parse::<u8>().unwrap();
            let b = line[3..5].parse::<u8>().unwrap();
            rules.push((a, b));
        } else {
            sum += right_order(&rules, line) as i32;
        }
    });
    sum
}

fn wrong_order(rules: &Vec<(u8, u8)>, pages_str: &str) -> u8 {
    let mut pages: Vec<u8> = pages_str.split(',').map(|i| i.parse::<u8>().unwrap()).collect();
    let mut isWrong = false;
    for (a, b) in rules {
        if pages.contains(a) && pages.contains(b) {
            let pos_a = pages.iter().position(|x| x == a).unwrap();
            let pos_b = pages.iter().position(|x| x == b).unwrap();
            if pos_a > pos_b{
                isWrong = true;
                break;
            }
        }
    }
    if !isWrong {return 0;}

    while isWrong {
        isWrong = false;
        for (a, b) in rules {
            if pages.contains(a) && pages.contains(b) {
                let pos_a = pages.iter().position(|x| x == a).unwrap();
                let pos_b = pages.iter().position(|x| x == b).unwrap();
                if pos_a > pos_b {
                    isWrong = true;
                    (pages[pos_a], pages[pos_b]) = (pages[pos_b], pages[pos_a]);
                }
            }
        }
    }
    pages[(pages.len() - 1) / 2]
}

pub(crate) fn part2(input: &str) -> i32 {

    let mut rules: Vec<(u8, u8)> = Vec::new();
    let mut is_header = true;
    let mut sum: i32 = 0;
    input.lines().for_each(|line| {
        if line.is_empty() {
            is_header = false;
        } else if is_header {
            let a = line[..2].parse::<u8>().unwrap();
            let b = line[3..5].parse::<u8>().unwrap();
            rules.push((a, b));
        } else {
            sum += wrong_order(&rules, line) as i32;
        }
    });
    sum
}
use std::collections::{HashSet};
//
// pub(crate) fn part1(input: &str) -> i32 {
//     let lines = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
//     let height = lines.len();
//     let width = lines[0].len();
//
//     let (mut pos_x, mut pos_y) = (0, 0);
//     for y in 0..height {
//         for x in 0..width {
//             if lines[y][x] == '^' {
//                 (pos_x, pos_y) = (x, y);
//                 break;
//             }
//         }
//     }
//
//     let mut visited = HashSet::new();
//     let mut direction: u8 = 0; // 0 up, 1 right, 2 down, 3 left
//
//     let get_next_pos = |x: usize, y: usize, direction: u8| -> (usize, usize) {
//         match direction {
//             0 => (x, y.wrapping_sub(1)),
//             1 => (x + 1, y),
//             2 => (x, y + 1),
//             3 => (x.wrapping_sub(1), y),
//             _ => panic!(),
//         }
//     };
//
//     let (mut next_x, mut next_y) = get_next_pos(pos_x, pos_y, direction);
//     while next_x < width
//         && next_y < height {
//         if lines[next_y][next_x] == '#' {
//             direction = (direction + 1) % 4;
//         } else {
//             visited.insert((pos_x, pos_y));
//             (pos_x, pos_y) = (next_x, next_y);
//         }
//         (next_x, next_y) = get_next_pos(pos_x, pos_y, direction);
//     }
//
//     visited.len() as i32 + 1
// }

pub(crate) fn part2(input: &str) -> i32 {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let height = lines.len();
    let width = lines[0].len();

    let (mut pos_x, mut pos_y) = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if lines[y][x] == '^' {
                (pos_x, pos_y) = (x, y);
                break;
            }
        }
    }

    let (start_x, start_y) = (pos_x, pos_y);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut direction: u8 = 0; // 0 up, 1 right, 2 down, 3 left

    let get_next_pos = |x: usize, y: usize, direction: u8| -> (usize, usize) {
        match direction {
            0 => (x, y.wrapping_sub(1)),
            1 => (x + 1, y),
            2 => (x, y + 1),
            3 => (x.wrapping_sub(1), y),
            _ => panic!(),
        }
    };

    let (mut next_x, mut next_y) = get_next_pos(pos_x, pos_y, direction);
    while next_x < width
        && next_y < height {
        if lines[next_y][next_x] == '#' {
            direction = (direction + 1) % 4;
        } else {
            visited.insert((pos_x, pos_y));
            (pos_x, pos_y) = (next_x, next_y);
        }
        (next_x, next_y) = get_next_pos(pos_x, pos_y, direction);
    }
    visited.insert((pos_x, pos_y));

    let does_path_repeat = |lines: &Vec<Vec<char>>| {
        let mut visited: HashSet<(usize, usize, u8)> = HashSet::new();
        let (mut pos_x, mut pos_y) = (start_x, start_y);
        let mut direction = 0;
        let (mut next_x, mut next_y) = get_next_pos(pos_x, pos_y, direction);
        while next_x < width
            && next_y < height {
            if lines[next_y][next_x] == '#' {
                direction = (direction + 1) % 4;
            } else {
                let z = (pos_x, pos_y, direction);
                if visited.contains(&z) {
                    return true;
                }
                visited.insert(z);
                (pos_x, pos_y) = (next_x, next_y);
            }
            (next_x, next_y) = get_next_pos(pos_x, pos_y, direction);
        }
        false
    };

    let mut count = 0;
    for (x, y) in visited {
        lines[y][x] = '#';
        if does_path_repeat(&lines) {
            count +=1;
            println!("{}, {}", x, y);
        }
        lines[y][x] = '.';
    }
    count
}

pub(crate) fn part1(input: &str) -> i32 {
    let trial = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let xmas = "XMAS";
    let samx = "SAMX";

    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut count = 0;
    for j in 0..width - 3 {
        for i in 0..height {
            if lines[i][j..j+4].iter().collect::<String>().eq_ignore_ascii_case(xmas)
                || lines[i][j..j+4].iter().collect::<String>().eq_ignore_ascii_case(samx){
                count += 1;
            }
        }
    }

    for j in 0..width {
        for i in 0..height - 3 {
            if lines[i..i+4].iter().map(|l| l[j]).collect::<String>().eq_ignore_ascii_case(xmas)
                || lines[i..i+4].iter().map(|l| l[j]).collect::<String>().eq_ignore_ascii_case(samx){
                count += 1;
            }
        }
    }

    for j in 0..width - 3 {
        for i in 0..height - 3 {
            if lines[i..i+4].iter().enumerate().map(|(k,l)| l[j+k]).collect::<String>().eq_ignore_ascii_case(xmas)
                || lines[i..i+4].iter().enumerate().map(|(k,l)| l[j+k]).collect::<String>().eq_ignore_ascii_case(samx){
                count += 1;
            }
            if lines[i..i+4].iter().enumerate().map(|(k,l)| l[j+3-k]).collect::<String>().eq_ignore_ascii_case(xmas)
                || lines[i..i+4].iter().enumerate().map(|(k,l)| l[j+3-k]).collect::<String>().eq_ignore_ascii_case(samx){
                count += 1;
            }
        }
    }

    count
}

pub(crate) fn part2(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let a = 'A';
    let m = 'M';
    let s = 'S';
    let mut count = 0;
    for j in 1..width - 1 {
        for i in 1..height - 1 {
            if lines[i][j].eq(&a) {
                if lines[i-1][j-1].eq(&m)
                    && lines[i-1][j+1].eq(&m)
                    && lines[i+1][j-1].eq(&s)
                    && lines[i+1][j+1].eq(&s) {
                    count += 1;
                } else if lines[i-1][j-1].eq(&m)
                    && lines[i-1][j+1].eq(&s)
                    && lines[i+1][j-1].eq(&m)
                    && lines[i+1][j+1].eq(&s) {
                    count += 1;
                } else if lines[i-1][j-1].eq(&s)
                    && lines[i-1][j+1].eq(&m)
                    && lines[i+1][j-1].eq(&s)
                    && lines[i+1][j+1].eq(&m) {
                    count += 1;
                } else if lines[i-1][j-1].eq(&s)
                    && lines[i-1][j+1].eq(&s)
                    && lines[i+1][j-1].eq(&m)
                    && lines[i+1][j+1].eq(&m) {
                    count += 1;
                }
            }
        }
    }
    count
}
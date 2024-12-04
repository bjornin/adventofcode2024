pub fn s1(input: &str) -> usize {
    let lines = parse(input);
    let mut count = 0;
    const WORD: &str = "MAS";
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] == "X" {
                for rd in -1..=1 {
                    for cd in -1..=1 {
                        if rd == 0 && cd == 0 {
                            continue;
                        }
                        let mut r = row as i32;
                        let mut c = col as i32;
                        let mut found = true;
                        for w in WORD.chars() {
                            r += rd;
                            c += cd;
                            if r < 0
                                || r >= lines.len() as i32
                                || c < 0
                                || c >= lines[row].len() as i32
                            {
                                found = false;
                                break;
                            }
                            if lines[r as usize][c as usize] != w.to_string() {
                                found = false;
                                break;
                            }
                        }
                        if found {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn s2(input: &str) -> usize {
    let lines = parse(input);
    let mut count = 0;
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] == "A" {
                if row == 0 || row == lines.len() - 1 || col == 0 || col == lines[row].len() - 1 {
                    continue;
                }
                let mut chars = ["S", "S", "M", "M"];
                for _ in 0..4 {
                    chars.rotate_left(1);
                    if lines[row - 1][col - 1] == chars[0]
                        && lines[row - 1][col + 1] == chars[1]
                        && lines[row + 1][col + 1] == chars[2]
                        && lines[row + 1][col - 1] == chars[3]
                    {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split("").filter(|&c| !c.is_empty()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
..X...
.SAMX.
.A..A.
XMAS.S
.X....
";

    const INPUT2: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 4);
    }
    #[test]
    fn test_s1_1() {
        let res = s1(INPUT2);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_s2() {
        let res = s2(INPUT2);
        assert_eq!(res, 9);
    }
}

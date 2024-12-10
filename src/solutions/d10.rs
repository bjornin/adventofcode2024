use std::collections::HashSet;

pub fn s1(input: &str) -> usize {
    let map = parse(input);
    let mut trails = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if val == 0 {
                let mut visited = HashSet::new();
                trails += find_path(row_idx as u32, col_idx as u32, &mut visited, &map, false);
            }
        }
    }
    trails
}

pub fn s2(input: &str) -> usize {
    let map = parse(input);
    let mut trails = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if val == 0 {
                let mut visited = HashSet::new();
                trails += find_path(row_idx as u32, col_idx as u32, &mut visited, &map, true);
            }
        }
    }
    trails
}

fn find_path(
    row: u32,
    col: u32,
    visited: &mut HashSet<(u32, u32)>,
    map: &Vec<Vec<u32>>,
    p2: bool,
) -> usize {
    if !p2 && !visited.insert((row, col)) {
        return 0;
    }
    let current_val = map[row as usize][col as usize];
    if current_val == 9 {
        return 1;
    }
    let nb = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut trails = 0;
    for n in nb {
        let next = (row as i32 + n.0, col as i32 + n.1);
        if (0..map.len() as i32).contains(&next.0)
            && (0..map[0].len() as i32).contains(&next.1)
            && map[next.0 as usize][next.1 as usize] == current_val + 1
        {
            trails += find_path(next.0 as u32, next.1 as u32, visited, map, p2);
        }
    }
    trails
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
0123
1234
8765
9876
";

    const INPUT2: &str = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_s1_2() {
        let res = s1(INPUT2);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_s2_2() {
        let res = s2(INPUT2);
        assert_eq!(res, 81);
    }
}

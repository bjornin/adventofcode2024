use std::collections::HashMap;

pub fn s1(input: &str) -> usize {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let map_ = parse(input);
    let mut start = map_
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &c)| {
                if c == "^" {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut visited = HashMap::new();
    let mut dir_iter = dirs.iter().cycle();
    'outer: loop {
        let dir = dir_iter.next().unwrap();
        'inner: loop {
            *visited.entry(start).or_insert(0) += 1;
            let next = (start.0 as i32 + dir.0, start.1 as i32 + dir.1);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= map_.len() as i32
                || next.1 >= map_[0].len() as i32
            {
                break 'outer;
            }
            if map_[next.0 as usize][next.1 as usize] == "#" {
                break 'inner;
            }
            start = next;
        }
    }
    visited.len()
}

pub fn s2(input: &str) -> usize {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let map_ = parse(input);
    let orig_start = map_
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &c)| {
                if c == "^" {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut visited: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut dir_iter = dirs.iter().cycle();
    let mut start = orig_start;
    'outer: loop {
        let dir = dir_iter.next().unwrap();
        'inner: loop {
            *visited.entry(start).or_insert(*dir) = *dir;
            let next = (start.0 as i32 + dir.0, start.1 as i32 + dir.1);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= map_.len() as i32
                || next.1 >= map_[0].len() as i32
            {
                break 'outer;
            }
            if map_[next.0 as usize][next.1 as usize] == "#" {
                break 'inner;
            }
            start = next;
        }
    }
    visited
        .keys()
        .filter(|&k| {
            if k == &orig_start {
                return false;
            }
            let mut new_map = map_.clone();
            new_map[k.0 as usize][k.1 as usize] = "#";
            let mut new_visited: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();
            let mut new_start = orig_start;
            let mut dir_iter = dirs.iter().cycle();
            loop {
                let dir = dir_iter.next().unwrap();
                'inner: loop {
                    if new_visited.contains_key(&(new_start, *dir)) {
                        return true;
                    } else {
                        new_visited.entry((new_start, *dir)).or_insert(0);
                    }
                    let next = (new_start.0 as i32 + dir.0, new_start.1 as i32 + dir.1);
                    if next.0 < 0
                        || next.1 < 0
                        || next.0 >= map_.len() as i32
                        || next.1 >= map_[0].len() as i32
                    {
                        return false;
                    }
                    if new_map[next.0 as usize][next.1 as usize] == "#" {
                        break 'inner;
                    }
                    new_start = next;
                }
            }
        })
        .count()
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split("").filter(|c| !c.is_empty()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 41);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 6);
    }
}

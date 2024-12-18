use std::{collections::HashSet, hash::Hash};

pub fn s1(input: &str) -> usize {
    let (mut map, moves) = parse(input);
    let mut rp = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &v)| if v == '@' { Some((r, c)) } else { None })
        })
        .unwrap();
    for m in moves {
        match m {
            '^' => rp = step(&mut map, rp, (-1, 0)),
            'v' => rp = step(&mut map, rp, (1, 0)),
            '<' => rp = step(&mut map, rp, (0, -1)),
            '>' => rp = step(&mut map, rp, (0, 1)),
            _ => {}
        }
    }
    // for row in map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &v)| if v == 'O' { Some(100 * r + c) } else { None })
        })
        .sum()
}

fn step(map: &mut Vec<Vec<char>>, pos: (usize, usize), dir: (i32, i32)) -> (usize, usize) {
    let (r, c) = pos;
    let (dr, dc) = dir;
    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
    if !(0..map.len() as i32).contains(&nr) && !(0..map[0].len() as i32).contains(&nc) {
        return pos;
    }
    let (nr, nc) = (nr as usize, nc as usize);
    if map[nr][nc] == 'O' {
        step(map, (nr, nc), dir);
    } else if map[nr][nc] == '#' {
        return pos;
    }
    if map[nr][nc] == '.' {
        let tile = map[r][c];
        map[r][c] = map[nr][nc];
        map[nr][nc] = tile;
        if tile == '@' {
            return (nr, nc);
        }
    }
    pos
}

pub fn s2(input: &str) -> usize {
    let (mut map, moves) = parse2(input);
    let (mut robot_r, mut robot_c) = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &v)| if v == '@' { Some((r, c)) } else { None })
        })
        .unwrap();
    for m in moves {
        let (dr, dc) = match m {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!(),
        };
        let mut visited = HashSet::new();
        let mut q = vec![(robot_r, robot_c)];

        while let Some((r, c)) = q.pop() {
            if !visited.insert((r, c)) {
                continue;
            }
            let (nr, nc) = (r + dr as usize, c + dc as usize);
            if !(0..map.len()).contains(&nr) && !(0..map[0].len()).contains(&nc) {
                continue;
            }
            match map[nr][nc] {
                'O' => q.push((nr, nc)),
                '[' => {
                    q.push((nr, nc));
                    q.push((nr, nc + 1));
                }
                ']' => {
                    q.push((nr, nc));
                    q.push((nr, nc - 1));
                }
                '#' => continue,
                _ => continue,
            }
        }
        println!("{:?}", visited);
    }
    // for row in map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }
    // map.iter()
    //     .enumerate()
    //     .flat_map(|(r, row)| {
    //         row.iter()
    //             .enumerate()
    //             .filter_map(move |(c, &v)| if v == 'O' { Some(100 * r + c) } else { None })
    //     })
    //     .sum()
    0
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (map_s, move_s) = input.split_once("\n\n").unwrap();
    let map = map_s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>();
    let moves = move_s.lines().flat_map(|l| l.chars()).collect();
    (map, moves)
}

fn parse2(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (map_s, move_s) = input.split_once("\n\n").unwrap();
    let map = map_s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '@' => "@.".chars().collect::<Vec<_>>(),
                    '#' => "##".chars().collect::<Vec<_>>(),
                    'O' => "[]".chars().collect::<Vec<_>>(),
                    '.' => "..".chars().collect::<Vec<_>>(),
                    _ => panic!(),
                })
                .collect()
        })
        .collect::<Vec<_>>();
    let moves = move_s.lines().flat_map(|l| l.chars()).collect();
    (map, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 2028);
    }

    const INPUT2: &str = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn test_s2() {
        let res = s2(INPUT2);
        assert_eq!(res, 105 + 207 + 306);
    }
}

use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    row: isize,
    col: isize,
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            row: self.row + rhs.row,
            col: self.col + rhs.row,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn s1(input: &str) -> usize {
    let map = parse(input);
    let r_max = map.len();
    let c_max = map[0].len();
    let mut antinodes: HashSet<Point> = HashSet::new();
    for c in ('0'..='9').chain('a'..='z').chain('A'..='Z') {
        let antennas: Vec<Point> = map
            .iter()
            .enumerate()
            .flat_map(|(row, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(col, a)| {
                        if *a == c {
                            Some(Point {
                                row: row as isize,
                                col: col as isize,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        if antennas.len() < 2 {
            continue;
        }
        for a in 0..antennas.len() - 1 {
            for b in a + 1..antennas.len() {
                let a = antennas[a];
                let b = antennas[b];
                let diff = b - a;
                let lower = a - diff;
                // let upper = b + diff;
                // Not sure why add is not working??
                let upper = Point {
                    row: b.row + diff.row,
                    col: b.col + diff.col,
                };
                if (0..r_max).contains(&(lower.row as usize))
                    && (0..c_max).contains(&(lower.col as usize))
                {
                    antinodes.insert(lower);
                }
                if (0..r_max).contains(&(upper.row as usize))
                    && (0..c_max).contains(&(upper.col as usize))
                {
                    antinodes.insert(upper);
                }
            }
        }
    }
    antinodes.len()
}

pub fn s2(input: &str) -> usize {
    let map = parse(input);
    let r_max = map.len();
    let c_max = map[0].len();
    let mut antinodes: HashSet<Point> = HashSet::new();
    for c in ('0'..='9').chain('a'..='z').chain('A'..='Z') {
        let antennas: Vec<Point> = map
            .iter()
            .enumerate()
            .flat_map(|(row, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(col, a)| {
                        if *a == c {
                            Some(Point {
                                row: row as isize,
                                col: col as isize,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        if antennas.len() < 2 {
            continue;
        }
        for a in 0..antennas.len() - 1 {
            for b in a + 1..antennas.len() {
                let a = antennas[a];
                let b = antennas[b];
                let diff = b - a;
                let mut lower = a - diff;
                antinodes.insert(a);
                antinodes.insert(b);
                while (0..r_max).contains(&(lower.row as usize))
                    && (0..c_max).contains(&(lower.col as usize))
                {
                    antinodes.insert(lower);
                    lower = lower - diff;
                }
                let mut upper = Point {
                    row: b.row + diff.row,
                    col: b.col + diff.col,
                };
                while (0..r_max).contains(&(upper.row as usize))
                    && (0..c_max).contains(&(upper.col as usize))
                {
                    antinodes.insert(upper);
                    upper = Point {
                        row: upper.row + diff.row,
                        col: upper.col + diff.col,
                    };
                }
            }
        }
    }
    antinodes.len()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 14);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 34);
    }
}

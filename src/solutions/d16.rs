use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost: isize,
    dir: (isize, isize),
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn s1(input: &str) -> usize {
    let maze = parse(input);
    let (mut start, mut end) = ((0, 0), (0, 0));
    for r in 0..maze.len() {
        for c in 0..maze[0].len() {
            match maze[r][c] {
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => {}
            }
        }
    }
    let mut q = BinaryHeap::new();
    let mut visited = HashMap::new();
    q.push(State {
        cost: 0,
        dir: (0, 1),
        position: start,
    });

    while let Some(State {
        cost,
        dir,
        position,
    }) = q.pop()
    {
        let &prev_cost = visited.get(&(position, dir)).unwrap_or(&isize::MAX);
        if prev_cost <= cost {
            continue;
        }
        visited
            .entry((position, dir))
            .and_modify(|c| *c = cost)
            .or_insert(cost);
        if position == end {
            return cost.unsigned_abs();
        }
        for neighbor in neighbors(&maze, position) {
            q.push(State {
                cost: cost + if neighbor.dir != dir { 1001 } else { 1 },
                ..neighbor
            });
        }
    }
    0
}

fn neighbors(maze: &[Vec<char>], (r, c): (usize, usize)) -> Vec<State> {
    let mut result = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for &(dr, dc) in &directions {
        let (nr, nc) = (r as isize + dr, c as isize + dc);
        if nr >= 0 && nr < maze.len() as isize && nc >= 0 && nc < maze[0].len() as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            if maze[nr][nc] != '#' {
                result.push(State {
                    cost: 0,
                    dir: (dr, dc),
                    position: (nr, nc),
                });
            }
        }
    }
    result
}

pub fn s2(input: &str) -> usize {
    let maze = parse(input);
    let (mut start, mut end) = ((0, 0), (0, 0));
    for r in 0..maze.len() {
        for c in 0..maze[0].len() {
            match maze[r][c] {
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => {}
            }
        }
    }
    let mut q = BinaryHeap::new();
    let mut visited = HashMap::new();
    q.push(State {
        cost: 0,
        dir: (0, 1),
        position: start,
    });

    while let Some(State {
        cost,
        dir,
        position,
    }) = q.pop()
    {
        if position == end {
            println!("{} {:?} {:?}", cost, position, dir);
            break;
        }
        for neighbor in neighbors(&maze, position) {
            let &(prev_cost, _) = visited
                .get(&(neighbor.position, neighbor.dir))
                .unwrap_or(&(isize::MAX, (0, 0)));
            let cost = cost + if neighbor.dir != dir { 1001 } else { 1 };
            if cost < prev_cost {
                q.push(State { cost, ..neighbor });
                visited
                    .entry((neighbor.position, neighbor.dir))
                    .and_modify(|(c, p)| {
                        *c = cost;
                        *p = position;
                    })
                    .or_insert((cost, position));
            }
        }
    }
    for (position, (cost, parent)) in &visited {
        println!("{:?}: cost = {}, parent = {:?}", position, cost, parent);
    }
    0
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const INPUT2: &str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    const INPUT3: &str = r"
##############
#...########E#
#.#.##.......#
#.#.##.#####.#
#.#..........#
#.####.#######
#.####.......#
#.##########.#
#S...........#
##############
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 7036);
    }
    #[test]
    fn test_s1_2() {
        let res = s1(INPUT2);
        assert_eq!(res, 11048);
    }
    #[test]
    fn test_s1_3() {
        let res = s1(INPUT3);
        assert_eq!(res, 5024);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 45);
    }
    #[test]
    fn test_s2_2() {
        let res = s2(INPUT2);
        assert_eq!(res, 64);
    }

    const INPUT4: &str = r"
#######
#....E#
#.##.##
#S...##
#######
";
    #[test]
    fn test_s2_4() {
        let res = s2(INPUT4);
        assert_eq!(res, 10);
    }
}

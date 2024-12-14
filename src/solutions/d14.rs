use std::collections::{HashMap, HashSet};

pub fn s1(input: &str, size: (usize, usize)) -> usize {
    let (width, height) = size;
    let robots = parse(input);
    let positions: Vec<(i32, i32)> = robots
        .iter()
        .map(|r| {
            (
                ((r.x + r.vx * 100) % width as i32 + width as i32) % width as i32,
                ((r.y + r.vy * 100) % height as i32 + height as i32) % height as i32,
            )
        })
        .collect();
    let mut quads = HashMap::new();
    positions.iter().for_each(|(x, y)| match (x, y) {
        (x, y) if *x < width as i32 / 2 && *y < height as i32 / 2 => {
            *quads.entry(1).or_insert(0) += 1;
        }
        (x, y) if *x > width as i32 / 2 && *y < height as i32 / 2 => {
            *quads.entry(2).or_insert(0) += 1;
        }
        (x, y) if *x < width as i32 / 2 && *y > height as i32 / 2 => {
            *quads.entry(3).or_insert(0) += 1;
        }
        (x, y) if *x > width as i32 / 2 && *y > height as i32 / 2 => {
            *quads.entry(4).or_insert(0) += 1;
        }
        _ => {}
    });
    quads.values().product()
}

pub fn s2(input: &str, size: (usize, usize)) -> usize {
    let (width, height) = size;
    let mut robots = parse(input);
    let mut rs = HashMap::new();
    for i in 1..width * height {
        robots = robots
            .iter()
            .map(|r| Robot {
                x: ((r.x + r.vx) % width as i32 + width as i32) % width as i32,
                y: ((r.y + r.vy) % height as i32 + height as i32) % height as i32,
                vx: r.vx,
                vy: r.vy,
            })
            .collect();
        let r_set = robots
            .iter()
            .map(|r| (r.x as usize, r.y as usize))
            .collect::<HashSet<_>>();
        rs.insert(i, r_set.len());
    }
    *rs.iter().max_by_key(|&(_, v)| v).unwrap().0
}

#[derive(Clone, Debug, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let (p, v) = s.split_once(" ").unwrap();
        let mut p_parts = p.split_terminator(['=', ','].as_ref());
        let mut v_parts = v.split_terminator(['=', ','].as_ref());
        p_parts.next();
        v_parts.next();
        Robot {
            x: p_parts.next().unwrap().parse().unwrap(),
            y: p_parts.next().unwrap().parse().unwrap(),
            vx: v_parts.next().unwrap().parse().unwrap(),
            vy: v_parts.next().unwrap().parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Robot::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT, (11, 7));
        assert_eq!(res, 12);
    }
}

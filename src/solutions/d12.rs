use std::collections::{HashMap, HashSet, VecDeque};

pub fn s1(input: &str) -> usize {
    let map = parse(input);
    let nb: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if visited.contains(&(r as i32, c as i32)) {
                continue;
            }
            let mut perimiter: HashMap<char, usize> = HashMap::new();
            let mut area: HashMap<char, usize> = HashMap::new();
            q.push_back((r as i32, c as i32));
            while let Some((rr, cc)) = q.pop_front() {
                if !visited.insert((rr, cc)) {
                    continue;
                }
                let current = map[rr as usize][cc as usize];
                *area.entry(current).or_default() += 1;
                let mut neighbors = 0;
                for (dr, dc) in nb.iter() {
                    let nr = rr + dr;
                    let nc = cc + dc;
                    if !(0..map.len()).contains(&(nr as usize))
                        || !(0..map[0].len()).contains(&(nc as usize))
                    {
                        continue;
                    }
                    if current == map[nr as usize][nc as usize] {
                        neighbors += 1;
                        if !visited.contains(&(nr, nc)) {
                            q.push_back((nr, nc));
                        }
                    }
                }
                *perimiter.entry(current).or_default() += 4 - neighbors;
            }
            sum += perimiter
                .keys()
                .map(|k| perimiter[k] * area[k])
                .sum::<usize>();
        }
    }
    sum
}

pub fn s2(input: &str) -> usize {
    let map = parse(input);
    let nb: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if visited.contains(&(r as i32, c as i32)) {
                continue;
            }
            let mut perimiter: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
            let mut area = 0;
            let mut sides = 0;
            q.push_back((r as i32, c as i32));
            while let Some((rr, cc)) = q.pop_front() {
                if !visited.insert((rr, cc)) {
                    continue;
                }
                let current = map[rr as usize][cc as usize];
                area += 1;
                for (dr, dc) in nb.iter() {
                    let nr = rr + dr;
                    let nc = cc + dc;
                    if !(0..map.len()).contains(&(nr as usize))
                        || !(0..map[0].len()).contains(&(nc as usize))
                    {
                        perimiter
                            .entry((rr, cc))
                            .and_modify(|v| v.push((*dr, *dc)))
                            .or_insert(vec![(*dr, *dc)]);
                        continue;
                    }
                    if current == map[nr as usize][nc as usize] {
                        if !visited.contains(&(nr, nc)) {
                            q.push_back((nr, nc));
                        }
                    } else {
                        perimiter
                            .entry((rr, cc))
                            .and_modify(|v| v.push((*dr, *dc)))
                            .or_insert(vec![(*dr, *dc)]);
                    }
                }
                if let Some(mut cp) = perimiter.get(&(rr, cc)).cloned() {
                    for (dr, dc) in nb.iter() {
                        let nr = rr + dr;
                        let nc = cc + dc;
                        if let Some(p) = perimiter.get(&(nr, nc)) {
                            cp.retain(|&(dr, dc)| !p.contains(&(dr, dc)));
                        }
                    }
                    sides += cp.len();
                }
            }
            sum += area * sides;
        }
    }
    sum
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
AAAA
BBCD
BBCC
EEEC
";

    const INPUT2: &str = r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const INPUT3: &str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 140);
    }
    #[test]
    fn test_s1_2() {
        let res = s1(INPUT2);
        assert_eq!(res, 772);
    }
    #[test]
    fn test_s1_3() {
        let res = s1(INPUT3);
        assert_eq!(res, 1930);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 80);
    }
    #[test]
    fn test_s2_2() {
        let res = s2(INPUT2);
        assert_eq!(res, 436);
    }
    #[test]
    fn test_s2_3() {
        let res = s2(INPUT3);
        assert_eq!(res, 1206);
    }

    const INPUT4: &str = r"
AA
AA
";
    #[test]
    fn test_s1_4() {
        let res = s2(INPUT4);
        assert_eq!(res, 16);
    }
}

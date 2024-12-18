use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn s1(input: &str, size: usize, mems: usize) -> usize {
    let corrupt_mem = parse(input);
    let partial_corrupt_mem = corrupt_mem[0..mems].to_vec();
    let start = (0, 0);
    let end = (size, size);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut q = BinaryHeap::new();
    let mut seen: HashSet<(usize, (usize, usize))> = HashSet::new();
    q.push((Reverse(0), start));
    while let Some((cost, (r, c))) = q.pop() {
        if (r, c) == end {
            return cost.0;
        }
        for (i, (dr, dc)) in dirs.iter().enumerate() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if !(0..=size).contains(&(nr as usize))
                || !(0..=size).contains(&(nc as usize))
                || partial_corrupt_mem.contains(&(nr as usize, nc as usize))
            {
                continue;
            }
            let n = (nr as usize, nc as usize);
            if seen.insert((i, n)) {
                q.push((Reverse(cost.0 + 1), n));
            }
        }
    }
    0
}

pub fn s2(input: &str, size: usize, mems: usize) -> (usize, usize) {
    let corrupt_mem = parse(input);
    let start = (0, 0);
    let end = (size, size);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut left = mems;
    let mut right = corrupt_mem.len();
    while left < right {
        let mid = left + (right - left) / 2;
        let partial_corrupt_mem = corrupt_mem[0..mid].to_vec();
        let mut q = BinaryHeap::new();
        let mut seen: HashSet<(usize, (usize, usize))> = HashSet::new();
        q.push((Reverse(0), start));
        let mut found = false;
        while let Some((cost, (r, c))) = q.pop() {
            if (r, c) == end {
                found = true;
                break;
            }
            for (i, (dr, dc)) in dirs.iter().enumerate() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if !(0..=size).contains(&(nr as usize))
                    || !(0..=size).contains(&(nc as usize))
                    || partial_corrupt_mem.contains(&(nr as usize, nc as usize))
                {
                    continue;
                }
                let n = (nr as usize, nc as usize);
                if seen.insert((i, n)) {
                    q.push((Reverse(cost.0 + 1), n));
                }
            }
        }
        if found {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left == mems {
        (0, 0)
    } else {
        corrupt_mem[left - 1]
    }
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT, 6, 12);
        assert_eq!(res, 22);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT, 6, 12);
        assert_eq!(res, (6, 1));
    }
}

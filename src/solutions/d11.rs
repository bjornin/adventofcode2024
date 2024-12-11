use std::collections::HashMap;

pub fn s1(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..25 {
        let mut new_stones = vec![];
        stones.iter().for_each(|s| {
            let sn = format!("{}", s);
            if *s == 0 {
                new_stones.push(1);
            } else if sn.len() % 2 == 0 {
                let (a, b) = sn.split_at(sn.len() / 2);
                new_stones.push(a.parse::<usize>().unwrap());
                new_stones.push(b.parse::<usize>().unwrap());
            } else {
                new_stones.push(s * 2024);
            }
        });
        stones = new_stones;
    }
    stones.len()
}

pub fn s2(input: &str) -> usize {
    let stones = parse(input);
    let mut hash_stones = stones
        .iter()
        .map(|n| (*n, 1))
        .collect::<HashMap<usize, usize>>();
    for _ in 0..75 {
        hash_stones = mutate(&hash_stones);
    }
    hash_stones.values().sum()
}

fn mutate(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();
    stones.iter().for_each(|(&k, &v)| {
        let sn = format!("{}", k);
        if k == 0 {
            new_stones.entry(1).and_modify(|e| *e += v).or_insert(v);
        } else if sn.len() % 2 == 0 {
            let (a, b) = sn.split_at(sn.len() / 2);
            new_stones
                .entry(a.parse::<usize>().unwrap())
                .and_modify(|e| *e += v)
                .or_insert(v);
            new_stones
                .entry(b.parse::<usize>().unwrap())
                .and_modify(|e| *e += v)
                .or_insert(v);
        } else {
            new_stones
                .entry(k * 2024)
                .and_modify(|e| *e += v)
                .or_insert(v);
        }
    });
    new_stones
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"125 17";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 55312);
    }
}

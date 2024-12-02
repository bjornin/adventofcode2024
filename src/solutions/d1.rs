use std::collections::HashMap;

pub fn s1(input: &str) -> usize {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

pub fn s2(input: &str) -> usize {
    let (left, right) = parse(input);
    let mut lookup_table: HashMap<usize, usize> = HashMap::new();
    right.iter().for_each(|num| {
        *lookup_table.entry(*num).or_insert(0) += 1;
    });
    left.iter()
        .filter(|num| lookup_table.contains_key(num))
        .map(|num| num * lookup_table.get(num).unwrap())
        .sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse::<usize>().unwrap());
            right.push(parts.next().unwrap().parse::<usize>().unwrap());
        });
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 31);
    }
}

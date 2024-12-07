pub fn s1(input: &str) -> usize {
    let rows = parse(input);
    rows.iter()
        .filter(|(r, nums)| calc(*r, nums))
        .map(|(r, _)| r)
        .sum()
}

fn calc(res: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return nums[0] == res;
    }
    let mut iter = nums.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let rest = iter.as_slice();
    let mut new_add: Vec<usize> = vec![first + second];
    new_add.extend(rest);

    let mut new_mul: Vec<usize> = vec![first * second];
    new_mul.extend(rest);

    calc(res, &new_add) || calc(res, &new_mul)
}

pub fn s2(input: &str) -> usize {
    let rows = parse(input);
    rows.iter()
        .filter(|(r, nums)| calc_2(*r, nums))
        .map(|(r, _)| r)
        .sum()
}

fn calc_2(res: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return nums[0] == res;
    }
    let mut iter = nums.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let rest = iter.as_slice();
    let mut new_add: Vec<usize> = vec![first + second];
    new_add.extend(rest);

    let mut new_mul: Vec<usize> = vec![first * second];
    new_mul.extend(rest);

    let mut new_comb: Vec<usize> = vec![format!("{}{}", first, second).parse::<usize>().unwrap()];
    new_comb.extend(rest);

    calc_2(res, &new_add) || calc_2(res, &new_mul) || calc_2(res, &new_comb)
}

type Row = (usize, Vec<usize>);

fn parse(input: &str) -> Vec<Row> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .fold(vec![], |acc: Vec<Row>, line| {
            let (r, n) = line.split_once(": ").unwrap();

            let nums = n
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut acc = acc;
            acc.push((r.parse::<usize>().unwrap(), nums));
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 3749);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 11387);
    }
}

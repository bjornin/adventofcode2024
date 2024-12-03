use regex::Regex;

pub fn s1(input: &str) -> usize {
    let nums = parse(input);
    nums.iter().map(|(n1, n2)| n1 * n2).sum()
}

pub fn s2(input: &str) -> usize {
    let donts = input.split("don't()");
    let first_bit = parse(donts.clone().next().unwrap());
    let first_num = first_bit.iter().map(|(n1, n2)| n1 * n2).sum::<usize>();
    donts
        .map(|dont| {
            let (_, dos) = dont.split_once("do()").unwrap_or(("", ""));
            let nums = parse(dos);
            nums.iter().map(|(n1, n2)| n1 * n2).sum::<usize>()
        })
        .sum::<usize>()
        + first_num
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((n1.parse::<usize>().unwrap(), n2.parse::<usize>().unwrap()));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 161);
    }

    const INPUT2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_s2() {
        let res = s2(INPUT2);
        assert_eq!(res, 48);
    }
}

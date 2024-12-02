pub fn s1(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|report| check_report(report)).count()
}

pub fn s2(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .filter(|report| {
            for i in 0..report.len() {
                let mut alt_report = report.to_vec();
                alt_report.remove(i);
                if check_report(&alt_report) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn check_report(report: &[usize]) -> bool {
    report.iter().is_sorted_by(|&a, &b| a < b && b - a <= 3)
        || report.iter().is_sorted_by(|&a, &b| a > b && a - b <= 3)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let reports = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|report| {
            report
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 4);
    }
    #[test]
    fn test2_s2() {
        let res = s2(r"45 47 48 51 54 56 54");
        assert_eq!(res, 1);
    }
}

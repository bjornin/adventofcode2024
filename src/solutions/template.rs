pub fn s1(input: &str) -> usize {
    0
}

pub fn s2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 0);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT2);
        assert_eq!(res, 0);
    }
}

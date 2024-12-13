// 26810

pub fn s1(input: &str) -> usize {
    let coords = parse(input);
    coords
        .iter()
        .filter_map(|[x, y]| {
            let (a, b) = intersect(*x, *y).unwrap_or((0, 0));
            let t_a = x.0 * a + x.1 * b;
            let t_b = y.0 * a + y.1 * b;
            if t_a == x.2 && t_b == y.2 {
                return Some((a * 3 + b) as usize);
            }
            None
        })
        .sum()
}

fn intersect(x: (i64, i64, i64), y: (i64, i64, i64)) -> Option<(i64, i64)> {
    let den = x.0 * y.1 - x.1 * y.0;
    if den == 0 {
        return None;
    }
    let a = (x.2 * y.1 - x.1 * y.2) / den;
    let b = (x.0 * y.2 - x.2 * y.0) / den;
    Some((a, b))
}

pub fn s2(input: &str) -> usize {
    let coords = parse(input);
    coords
        .iter()
        .filter_map(|[x, y]| {
            let xn = (x.0, x.1, x.2 + 10000000000000);
            let yn = (y.0, y.1, y.2 + 10000000000000);
            let (a, b) = intersect(xn, yn).unwrap_or((0, 0));
            let t_a = x.0 * a + x.1 * b;
            let t_b = y.0 * a + y.1 * b;
            if t_a == xn.2 && t_b == yn.2 {
                return Some((a * 3 + b) as usize);
            }
            None
        })
        .sum()
}

fn parse(input: &str) -> Vec<[(i64, i64, i64); 2]> {
    input
        .split("\n\n")
        .map(|section| {
            let coords = section
                .lines()
                .filter_map(|line| line.split_once(':'))
                .flat_map(|(_, coords)| coords.split(','))
                .filter_map(|coord| {
                    coord
                        .split_once(['+', '='])
                        .map(|(_, num)| num.trim().parse::<i64>())
                })
                .collect::<Vec<_>>();
            [
                (
                    coords[0].clone().unwrap(),
                    coords[2].clone().unwrap(),
                    coords[4].clone().unwrap(),
                ),
                (
                    coords[1].clone().unwrap(),
                    coords[3].clone().unwrap(),
                    coords[5].clone().unwrap(),
                ),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 480);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 0);
    }
}

use std::collections::HashMap;

pub fn s1(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .filter(|u| {
            u.windows(2).all(|w| {
                if let Some(r) = rules.get(&w[0]) {
                    r.contains(&w[1])
                } else {
                    false
                }
            })
        })
        .map(|u| u.get(u.len() / 2).unwrap())
        .sum()
}

pub fn s2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .filter(|u| {
            !u.windows(2).all(|w| {
                if let Some(r) = rules.get(&w[0]) {
                    r.contains(&w[1])
                } else {
                    false
                }
            })
        })
        .map(|u| {
            let cu = corrected_update(&rules, u.to_vec());
            *cu.get(cu.len() / 2).unwrap()
        })
        .sum()
}

fn corrected_update(rules: &HashMap<usize, Vec<usize>>, update: Vec<usize>) -> Vec<usize> {
    let mut u = update.clone();
    let mut is_updated = true;
    while is_updated {
        is_updated = false;
        for i in 0..u.len() - 1 {
            if let Some(r) = rules.get(&u[i + 1]) {
                if r.contains(&u[i]) {
                    u.swap(i, i + 1);
                    is_updated = true;
                }
            }
        }
    }
    u.to_vec()
}

fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules = HashMap::new();
    let mut updates = vec![];
    input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        if l.contains(",") {
            let u = l.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
            updates.push(u);
        } else if l.contains("|") {
            let (k, v) = l.split_once("|").unwrap();
            let key = k.parse::<usize>().unwrap();
            let val = v.parse::<usize>().unwrap();
            rules
                .entry(key)
                .and_modify(|f: &mut Vec<usize>| f.push(val))
                .or_insert(vec![val]);
        } else {
            unimplemented!();
        }
    });
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 143);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 123);
    }
}

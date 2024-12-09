use std::collections::HashMap;

pub fn s1(input: &str) -> usize {
    let disk = parse(input);
    let chunks: Vec<&[u32]> = disk.chunks(2).collect();
    let mut free_size = 0;
    let (files, free) = chunks.iter().enumerate().fold(
        (Vec::new(), Vec::new()),
        |(mut acc_files, mut acc_free), (id, f)| {
            acc_files.push(vec![id as u32; f[0] as usize]);
            if f.len() > 1 {
                free_size += f[1] as usize;
                acc_free.push(f[1]);
            }
            (acc_files, acc_free)
        },
    );
    let final_size: usize = disk.iter().sum::<u32>() as usize - free_size;
    let mut files_rev: Vec<u32> = files.iter().rev().flatten().copied().collect();
    let mut files_rev_it = files_rev.iter_mut();
    let mut res: Vec<Vec<u32>> = Vec::new();
    for (i, f) in files.iter().enumerate() {
        res.push(f.to_vec());
        if let Some(free) = free.get(i) {
            let fill: Vec<u32> = files_rev_it
                .by_ref()
                .take(*free as usize)
                .map(|n| *n)
                .collect();
            res.push(fill);
        }
    }
    let trimmed = res
        .iter()
        .flatten()
        .copied()
        .take(final_size)
        .collect::<Vec<u32>>();

    trimmed.iter().enumerate().fold(0, |mut acc, (i, n)| {
        acc += i * (*n as usize);
        acc
    })
}

pub fn s2(input: &str) -> usize {
    let disk = parse(input);
    let mut start = 0;
    let (mut files, mut free) = disk.chunks(2).enumerate().fold(
        (
            HashMap::<usize, Vec<usize>>::new(),
            HashMap::<usize, Vec<usize>>::new(),
        ),
        |(mut acc_files, mut acc_free), (id, f)| {
            let mut end = start + (f[0]) as usize;
            acc_files.insert(id, (start..end).collect());
            start = end;
            if f.len() > 1 {
                end = start + (f[1]) as usize;
                acc_free.insert(id, (start..end).collect());
                start = end;
            }
            (acc_files, acc_free)
        },
    );
    let mut keys: Vec<_> = files.keys().cloned().collect();
    keys.sort();
    keys.reverse();
    for k in keys {
        let v = files.get_mut(&k).unwrap();
        for i in 0..k {
            if let Some(f) = free.get_mut(&i) {
                if v.len() <= f.len() {
                    let drained: Vec<usize> = f.drain(..v.len()).collect();
                    *v = drained;
                    break;
                }
            }
        }
    }
    files
        .iter()
        .map(|(k, v)| *k * v.iter().sum::<usize>())
        .sum()
}

fn parse(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, 1928);
    }
    #[test]
    fn test_s2() {
        let res = s2(INPUT);
        assert_eq!(res, 2858);
    }
}

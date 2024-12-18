#[derive(Debug, Clone)]
struct Program {
    a: usize,
    b: usize,
    c: usize,
    out: Vec<usize>,
}

impl Program {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            out: Vec::new(),
        }
    }

    fn run(&mut self, instr: &[usize]) {
        let mut pointer = 0;
        while let Some(opcode) = instr.get(pointer) {
            let operand = instr.get(pointer + 1).unwrap();
            match opcode {
                0 => self.adv(*operand),
                1 => self.bxl(*operand),
                2 => self.bst(*operand),
                3 => {
                    if let Some(p) = self.jnz(*operand) {
                        pointer = p;
                        continue;
                    }
                }
                4 => self.bxc(*operand),
                5 => self.out(*operand),
                6 => self.bdv(*operand),
                7 => self.cdv(*operand),
                _ => panic!("Invalid operand"),
            }
            pointer += 2;
        }
    }

    fn stdout(&self) -> String {
        self.out
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn adv(&mut self, operand: usize) {
        self.a /= 2usize.pow(self.combo(operand) as u32);
    }

    fn bxl(&mut self, operand: usize) {
        self.b ^= operand;
    }

    fn bst(&mut self, operand: usize) {
        self.b = self.combo(operand) % 8;
    }

    fn jnz(&self, operand: usize) -> Option<usize> {
        if self.a != 0 {
            Some(operand)
        } else {
            None
        }
    }

    fn bxc(&mut self, _: usize) {
        self.b ^= self.c;
    }

    fn out(&mut self, operand: usize) {
        self.out.push(self.combo(operand) % 8);
    }

    fn bdv(&mut self, operand: usize) {
        self.b = self.a / (2usize.pow(self.combo(operand) as u32));
    }

    fn cdv(&mut self, operand: usize) {
        self.c = self.a / (2usize.pow(self.combo(operand) as u32));
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("reserved"),
            _ => panic!("Invalid operand"),
        }
    }
}

pub fn s1(input: &str) -> String {
    let (mut program, instr) = parse(input);
    program.run(&instr);
    program.stdout()
}

pub fn s2(input: &str) -> usize {
    let (_, instr) = parse(input);
    for i in 1.. {
        let mut program = Program::new(i, 0, 0);
        program.run(&instr);
        if program.out == instr {
            return i;
        }
    }
    0
}

fn parse(input: &str) -> (Program, Vec<usize>) {
    let parts = input.split_once("\n\n").unwrap();
    let mut lines = parts.0.trim().lines();
    let a = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let p = Program::new(a, b, c);
    let i = parts
        .1
        .split_whitespace()
        .last()
        .map(|x| x.split(',').map(|n| n.parse().unwrap()))
        .unwrap()
        .collect();
    (p, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_s1() {
        let res = s1(INPUT);
        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }

    const INPUT2: &str = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_s2() {
        let res = s2(INPUT2);
        assert_eq!(res, 117440);
    }

    #[test]
    fn test_adv() {
        let mut r = Program::new(2, 0, 0);
        r.adv(1);
        assert_eq!(r.a, 1);
    }
    #[test]
    fn test_combo() {
        let r = Program::new(4, 5, 6);
        assert_eq!(r.combo(0), 0);
        assert_eq!(r.combo(1), 1);
        assert_eq!(r.combo(2), 2);
        assert_eq!(r.combo(3), 3);
        assert_eq!(r.combo(4), 4);
        assert_eq!(r.combo(5), 5);
        assert_eq!(r.combo(6), 6);
    }

    #[test]
    fn test_bxl() {
        let mut r = Program::new(0, 4, 0);
        r.bxl(1);
        assert_eq!(r.b, 5);
        r.bxl(2);
        assert_eq!(r.b, 7);
    }

    #[test]
    fn test_bst() {
        let mut r = Program::new(9, 0, 0);
        r.bst(1);
        assert_eq!(r.b, 1);
        r.bst(4);
        assert_eq!(r.b, 1);
    }

    #[test]
    fn test_bxc() {
        let mut r = Program::new(0, 4, 5);
        r.bxc(1);
        assert_eq!(r.b, 1);
        r.bxc(2);
        assert_eq!(r.b, 4);
    }

    #[test]
    fn test_out() {
        let mut r = Program::new(0, 4, 10);
        r.out(5);
        assert_eq!(r.stdout(), "4");
        r.out(1);
        assert_eq!(r.stdout(), "4,1");
    }

    #[test]
    fn test_bdv() {
        let mut r = Program::new(2, 0, 0);
        r.bdv(1);
        assert_eq!(r.b, 1);
    }

    #[test]
    fn test_cdv() {
        let mut r = Program::new(2, 0, 0);
        r.cdv(1);
        assert_eq!(r.c, 1);
    }

    #[test]
    fn test_instruction_1() {
        let mut p = Program::new(0, 0, 9);
        let instr = vec![2, 6];
        p.run(&instr);
        assert_eq!(p.b, 1);
    }

    #[test]
    fn test_instruction_2() {
        let mut p = Program::new(10, 0, 0);
        let instr = vec![5, 0, 5, 1, 5, 4];
        p.run(&instr);
        assert_eq!(p.stdout(), "0,1,2");
    }

    #[test]
    fn test_instruction_3() {
        let mut p = Program::new(2024, 0, 0);
        let instr = vec![0, 1, 5, 4, 3, 0];
        p.run(&instr);
        assert_eq!(p.stdout(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(p.a, 0);
    }

    #[test]
    fn test_instruction_4() {
        let mut p = Program::new(0, 29, 0);
        let instr = vec![1, 7];
        p.run(&instr);
        assert_eq!(p.b, 26);
    }

    #[test]
    fn test_instruction_5() {
        let mut p = Program::new(0, 2024, 43690);
        let instr = vec![4, 0];
        p.run(&instr);
        assert_eq!(p.b, 44354);
    }
}

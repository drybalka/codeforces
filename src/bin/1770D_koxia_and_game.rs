use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, stdout, BufWriter, Write},
};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

const MODULO: i64 = 998244353;

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = scan.next::<usize>();
    for _ in 0..t {
        let n = scan.next::<usize>();
        let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
        let b: Vec<usize> = (0..n).map(|_| scan.next()).collect();

        let mut abs: Vec<_> = std::iter::zip(a, b).collect();
        let mut fixed: HashSet<usize> = HashSet::with_capacity(n);
        let mut to_fix: VecDeque<usize> = VecDeque::with_capacity(n);

        abs.retain(|(a, b)| {
            if a == b {
                to_fix.push_back(*a);
                false
            } else {
                true
            }
        });

        let mut factor = 1;
        for _ in 0..to_fix.len() {
            factor = (factor * n as i64) % MODULO;
        }

        let res = calculate(&mut abs, &mut fixed, &mut to_fix);

        writeln!(out, "{:?}", (res * factor) % MODULO).ok();
        // writeln!(out, "{:?}", abs).ok();
        // writeln!(out, "{:?}", fixed).ok();
        // writeln!(out, "{:?}", to_fix).ok();
    }
}

fn calculate(
    abs: &mut Vec<(usize, usize)>,
    fixed: &mut HashSet<usize>,
    to_fix: &mut VecDeque<usize>,
) -> i64 {
    // dbg!(&abs, &fixed, &to_fix);
    if simplify(abs, fixed, to_fix) {
        if let Some((a, b)) = abs.pop() {
            let mut abs2 = abs.clone();
            let mut fixed2 = fixed.clone();
            (calculate(abs, fixed, &mut VecDeque::from([a]))
                + calculate(&mut abs2, &mut fixed2, &mut VecDeque::from([b])))
                % MODULO
        } else {
            1
        }
    } else {
        0
    }
}

fn simplify(
    abs: &mut Vec<(usize, usize)>,
    fixed: &mut HashSet<usize>,
    to_fix: &mut VecDeque<usize>,
) -> bool {
    if let Some(next) = to_fix.pop_front() {
        if fixed.contains(&next) {
            false
        } else {
            fixed.insert(next);
            abs.retain(|(a, b)| {
                if *a == next {
                    to_fix.push_back(*b);
                    false
                } else if *b == next {
                    to_fix.push_back(*a);
                    false
                } else {
                    true
                }
            });
            simplify(abs, fixed, to_fix)
        }
    } else {
        true
    }
}

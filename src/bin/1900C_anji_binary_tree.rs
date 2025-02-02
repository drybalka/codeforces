use core::panic;
use std::io::{stdin, stdout, BufWriter, Write};

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

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = scan.next::<u64>();
    for _ in 0..t {
        let res = solve(&mut scan);
        writeln!(out, "{:?}", res).ok();
    }
}

fn solve(scan: &mut Scanner) -> usize {
    let n = scan.next::<usize>();
    let s = scan.next::<String>();
    let s: Vec<char> = s.chars().collect();

    let mut l: Vec<Option<usize>> = Vec::with_capacity(n);
    let mut r: Vec<Option<usize>> = Vec::with_capacity(n);

    for _ in 0..n {
        l.push(match scan.next::<usize>() {
            0 => None,
            x => Some(x - 1),
        });
        r.push(match scan.next::<usize>() {
            0 => None,
            x => Some(x - 1),
        });
    }

    let mut res = 0;
    let mut roots: Vec<usize> = vec![0];
    loop {
        let mut new_roots: Vec<usize> = Vec::new();
        for &root in roots.iter() {
            let mut current = root;
            loop {
                if l[current].is_none() && r[current].is_none() {
                    return res;
                }
                match s[current] {
                    'U' => {
                        if let Some(v) = l[current] {
                            new_roots.push(v);
                        }
                        if let Some(v) = r[current] {
                            new_roots.push(v);
                        }
                        break;
                    }
                    'L' => {
                        if let Some(v) = r[current] {
                            new_roots.push(v);
                        }
                        if let Some(v) = l[current] {
                            current = v;
                        } else {
                            break;
                        }
                    }
                    'R' => {
                        if let Some(v) = l[current] {
                            new_roots.push(v);
                        }
                        if let Some(v) = r[current] {
                            current = v;
                        } else {
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        res += 1;
        roots = new_roots;
    }
}

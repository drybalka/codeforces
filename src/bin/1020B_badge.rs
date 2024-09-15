use std::{
    collections::HashSet,
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

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();
    let p: Vec<usize> = (0..n).map(|_| scan.next()).collect();

    let mut pp: Vec<usize> = (1..=n).collect();
    for pi in pp.iter_mut() {
        for _ in 0..n {
            *pi = p[*pi - 1];
        }
    }

    let p_stable: HashSet<_> = pp.into_iter().collect();

    let mut pp: Vec<usize> = (1..=n).collect();
    for pi in pp.iter_mut() {
        while !p_stable.contains(pi) {
            *pi = p[*pi - 1];
        }
    }

    for pi in pp.iter() {
        write!(out, "{} ", *pi).ok();
    }
}

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
        let a = scan.next::<u64>();
        let b = scan.next::<u64>();

        writeln!(out, "{}", 2).ok();
        writeln!(out, "{} {}", 1, b - 1).ok();
        writeln!(out, "{} {}", a, b).ok();
    }
}

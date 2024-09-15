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

    let t = scan.next::<usize>();
    for _ in 0..t {
        let n = scan.next::<u64>();
        let mut l = u64::MIN;
        let mut r = (u64::MAX as f64).sqrt() as u64;
        while r - l > 1 {
            let mid = (l + r) / 2;
            if mid * (mid - 1) > 2 * n {
                r = mid;
            } else {
                l = mid;
            }
        }
        let different = l;
        let total = n - different * (different - 1) / 2 + different;
        writeln!(out, "{}", total).ok();
    }
}

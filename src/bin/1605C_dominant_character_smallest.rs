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
        let _ = scan.next::<usize>();
        let str = scan.next::<String>();

        let mut res = -1;
        if str.contains("aa") {
            res = 2;
        } else if str.contains("aba") || str.contains("aca") {
            res = 3;
        } else if str.contains("abca") || str.contains("acba") {
            res = 4;
        } else if str.contains("abbacca") || str.contains("accabba") {
            res = 7;
        }

        writeln!(out, "{:?}", res).ok();
    }
}

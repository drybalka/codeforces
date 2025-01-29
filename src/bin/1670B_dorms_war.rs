use std::{
    io::{stdin, stdout, BufWriter, Write},
    str::Chars,
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

    let t = scan.next::<usize>();
    for _ in 0..t {
        let _ = scan.next::<usize>();
        let s = scan.next::<String>();
        let k = scan.next::<usize>();
        let cs: Vec<char> = (0..k).map(|_| scan.next()).collect();

        fn gap(s: &mut Chars, cs: Vec<char>, max_gap: usize) -> usize {
            if let Some(g) = s.position(|c| cs.contains(&c)) {
                gap(s, cs, max_gap.max(g + 1))
            } else {
                max_gap
            }
        }

        let mut chars = s.chars();
        chars.next();
        let max_gap = gap(&mut chars, cs, 0);
        writeln!(out, "{:?}", max_gap).ok();
    }
}

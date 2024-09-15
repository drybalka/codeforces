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
        let n = scan.next::<usize>();
        let mut max_gap = vec![0; n];
        let mut last_seen = vec![0; n];

        for i in 1..=n {
            let a = scan.next::<usize>() - 1;
            max_gap[a] = max_gap[a].max(i - last_seen[a] - 1);
            last_seen[a] = i;
        }

        for i in 0..n {
            max_gap[i] = max_gap[i].max(n - last_seen[i]);
        }

        let mut answ: Vec<i32> = vec![-1; n];
        for i in 0..n {
            for j in max_gap[i]..n {
                if answ[j] != -1 {
                    break;
                }
                answ[j] = 1 + i as i32;
            }
        }

        for a in answ.iter() {
            write!(out, "{} ", a).ok();
        }
        writeln!(out).ok();
    }
}

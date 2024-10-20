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
        let k = scan.next::<usize>();
        let _ = scan.next::<usize>();
        let s: Vec<usize> = scan
            .next::<String>()
            .chars()
            .map(|c| (c as u8 - b'a').into())
            .collect();

        let mut acc = Vec::new();

        if go(n, k, &s, &mut acc) {
            writeln!(out, "YES").ok();
        } else {
            writeln!(out, "NO").ok();
            let missing: String = acc
                .iter()
                .map(|c| std::char::from_u32(*c as u32 + 'a' as u32).unwrap())
                .collect();
            writeln!(out, "{}", missing).ok();
        }
    }
}

fn go(n: usize, k: usize, s: &[usize], acc: &mut Vec<usize>) -> bool {
    if n == 0 {
        true
    } else {
        let mut first_occ: Vec<usize> = vec![usize::MAX; k];
        for (i, c) in s.iter().enumerate() {
            if first_occ[*c] > i {
                first_occ[*c] = i;
            }
        }
        let (c, &max_i) = first_occ
            .iter()
            .enumerate()
            .max_by_key(|(_, &i)| i)
            .unwrap();
        if max_i == usize::MAX {
            acc.extend(vec![c; n]);
            false
        } else {
            acc.push(c);
            go(n - 1, k, &s[(max_i + 1)..], acc)
        }
    }
}

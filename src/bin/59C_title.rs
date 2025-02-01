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

    let k = scan.next::<usize>();
    let title = scan.next::<String>();

    let mut used: Vec<bool> = vec![false; k];
    let mut v: Vec<_> = title.chars().map(to_alpha).collect();
    let len = v.len();
    for i in 0..len {
        if v[i] == u8::MAX {
            v[i] = v[len - i - 1];
        }
        let letter = v[i] as usize;
        if letter < k {
            used[letter] = true;
        }
    }

    let mut rest: Vec<usize> = used
        .iter()
        .enumerate()
        .filter(|(_, &b)| !b)
        .map(|(i, _)| i)
        .collect();

    let mid = if len % 2 == 0 { len / 2 } else { len / 2 + 1 };
    for i in (0..mid).rev() {
        if v[i] != v[len - i - 1] {
            writeln!(out, "IMPOSSIBLE").ok();
            return;
        } else if v[i] == u8::MAX {
            let next = rest.pop().unwrap_or(0);
            v[i] = next as u8;
            v[len - i - 1] = next as u8;
        }
    }

    if rest.is_empty() {
        let res: String = v.iter().map(from_alpha).collect();
        writeln!(out, "{}", res).ok();
    } else {
        writeln!(out, "IMPOSSIBLE").ok();
    }
}

fn to_alpha(c: char) -> u8 {
    if c == '?' {
        u8::MAX
    } else {
        c as u8 - b'a'
    }
}

fn from_alpha(&c: &u8) -> char {
    if c == u8::MAX {
        '?'
    } else {
        (c + b'a') as char
    }
}

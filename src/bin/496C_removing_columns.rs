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

fn is_ordered(v: &[char]) -> bool {
    v.iter().zip(v.iter().skip(1)).all(|(a, b)| a <= b)
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();
    let m = scan.next::<usize>();
    let rows: Vec<Vec<char>> = (0..n)
        .map(|_| scan.next::<String>().chars().collect())
        .collect();

    let mut res = 0;
    let mut slices: Vec<Slice> = vec![Slice { l: 0, r: n }];

    for i in 0..m {
        // dbg!(&res);
        // dbg!(&slices);
        let cols: Vec<Vec<char>> = slices
            .iter()
            .map(|s| rows[s.l..s.r].iter().map(|r| r[i]).collect::<Vec<_>>())
            .collect();

        if !cols.iter().all(|col| is_ordered(col)) {
            res += 1;
        } else {
            slices = cols
                .iter()
                .zip(slices)
                .flat_map(|(c, s)| {
                    let mut new_slices: Vec<Slice> = Vec::new();
                    let mut l = s.l;
                    let mut cc = c[0];
                    for r in s.l..s.r {
                        if cc != c[r - s.l] {
                            new_slices.push(Slice { l, r });
                            l = r;
                            cc = c[r - s.l];
                        }
                    }
                    new_slices.push(Slice { l, r: s.r });
                    new_slices
                        .into_iter()
                        .filter(|s| s.r - s.l > 1)
                        .collect::<Vec<_>>()
                })
                .collect();
        }
    }

    writeln!(out, "{:?}", res).ok();
}

#[derive(Debug)]
struct Slice {
    l: usize,
    r: usize,
}

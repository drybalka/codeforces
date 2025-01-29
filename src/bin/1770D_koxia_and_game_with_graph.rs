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

const MODULO: i64 = 998244353;

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = scan.next::<usize>();
    for _ in 0..t {
        let n = scan.next::<usize>();
        let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
        let b: Vec<usize> = (0..n).map(|_| scan.next()).collect();

        let mut res = 1;
        let mut visited: Vec<bool> = vec![false; n];
        let mut singles: Vec<bool> = vec![false; n];

        let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];
        std::iter::zip(a, b)
            .map(|(a, b)| (a - 1, b - 1))
            .for_each(|(a, b)| {
                if a == b {
                    if singles[a] {
                        res = 0
                    } else {
                        singles[a] = true;
                        res = (res * n as i64) % MODULO;
                    }
                } else {
                    edges[a].push(b);
                    edges[b].push(a);
                }
            });

        if res != 0 {
            for (i, &is_single) in singles.iter().enumerate() {
                if is_single {
                    traverse(&edges, &mut visited, i);
                }
            }

            for i in 0..n {
                if !visited[i] {
                    let cycled = traverse(&edges, &mut visited, i);
                    if cycled {
                        res = (res * 2) % MODULO;
                    } else {
                        res = 0;
                        break;
                    }
                }
            }
        }

        writeln!(out, "{:?}", res).ok();
        // writeln!(out, "{:?}", edges).ok();
        // writeln!(out, "{:?}", fixed).ok();
        // writeln!(out, "{:?}", to_fix).ok();
    }
}

fn traverse(edges: &[Vec<usize>], visited: &mut [bool], i: usize) -> bool {
    let mut cycled = false;
    let mut stack = vec![(i, i)];
    while let Some((prev, next)) = stack.pop() {
        if visited[next] {
            cycled = true;
        } else {
            visited[next] = true;
            for &j in &edges[next] {
                if j != prev {
                    stack.push((next, j));
                }
            }
        }
    }
    cycled
}

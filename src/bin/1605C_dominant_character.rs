use std::{
    cmp::max,
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

    let t = scan.next::<usize>();
    for _ in 0..t {
        let n = scan.next::<usize>();
        let str = scan.next::<String>();

        let mut res: i32 = -1;
        let mut lhs: Vec<i32> = vec![n as i32; n + 2]; // how many chars from left needs to be deleted to improve the final y by index
        lhs[0] = 0;
        let mut rhs: Vec<i32> = vec![0; n + 2]; // index of char where the last occurence of y happened
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;

        for (x, char) in str.chars().enumerate() {
            let x = x as i32;
            if char == 'a' {
                a += 1;
            } else if char == 'b' {
                b += 1;
            } else if char == 'c' {
                c += 1;
            }
            let y = max(b, c) - a;
            if y > 0 {
                let y = y as usize;
                if lhs[y] > x + 1 {
                    lhs[y] = x + 1;
                }
            }
            if y + 1 >= 0 {
                let y = (y + 1) as usize;
                rhs[y] = x;
            }
        }

        let last_y = max(b, c) - a;
        if last_y < 0 {
            res = n as i32;
        } else {
            for y in 0..=last_y {
                let y = y as usize;
                let local_res = rhs[y] - lhs[y];
                if local_res > res {
                    res = local_res;
                }
            }
        }
        if res < 2 {
            res = -1;
        }
        writeln!(out, "{:?}", a).ok();
        writeln!(out, "{:?}", b).ok();
        writeln!(out, "{:?}", c).ok();
        writeln!(out, "{:?}", lhs).ok();
        writeln!(out, "{:?}", rhs).ok();
        writeln!(out, "{:?}", res).ok();
        // let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    }

    // writeln!(out, "{:?}", a).ok();
}

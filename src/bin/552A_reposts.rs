use std::{
    collections::HashMap,
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

    let n = scan.next::<usize>();
    let mut map: HashMap<String, i32> = HashMap::with_capacity(n + 1);
    map.insert("polycarp".into(), 1);

    for _ in 0..n {
        let dst = scan.next::<String>().to_lowercase();
        scan.next::<String>();
        let src = scan.next::<String>().to_lowercase();

        let depth = map.get(&src).unwrap();
        map.insert(dst, depth + 1);
    }

    let res = map.values().max().unwrap();
    writeln!(out, "{:?}", res).ok();
}

use core::panic;
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

    let s = scan.next::<String>();
    let s = simplify(&s);
    // dbg!(&s);
    let mut res = compute(&s);

    let mut left: Vec<usize> = vec![0];
    let mut right: Vec<usize> = Vec::new();

    s.match_indices('*').for_each(|(i, _)| {
        left.push(i + 1);
        right.push(i);
    });
    right.push(s.len());

    for &l in &left {
        for &r in &right {
            if r > l {
                // dbg!(&s[l..r]);
                let paren = compute(&s[l..r]);
                let new_res = compute(&format!("{}{}{}", &s[..l], paren, &s[r..]));
                if new_res > res {
                    res = new_res;
                }
            }
        }
    }

    // writeln!(out, "{:?}", s).ok();
    // writeln!(out, "{:?}", left).ok();
    // writeln!(out, "{:?}", right).ok();
    writeln!(out, "{:?}", res).ok();
}

fn simplify(expr: &str) -> String {
    let (expr, sum) = expr
        .split('+')
        .fold((String::new(), 0), |(expr, sum), elem| {
            if !elem.contains('*') {
                (expr, sum + elem.parse::<u64>().unwrap())
            } else {
                match (expr.is_empty(), sum == 0) {
                    (true, true) => (elem.to_string(), 0),
                    (true, false) => (format!("{}+{}", sum, elem), 0),
                    (false, true) => (format!("{}+{}", expr, elem), 0),
                    (false, false) => (format!("{}+{}+{}", expr, sum, elem), 0),
                }
            }
        });

    match (expr.is_empty(), sum == 0) {
        (true, true) => panic!(),
        (true, false) => format!("{}", sum),
        (false, true) => expr.to_string(),
        (false, false) => format!("{}+{}", expr, sum),
    }
}

// fn right_positions(expr: &str) -> Vec<usize> {
//     let mut res: Vec<usize> = expr
//         .split('+')
//         .scan(0, |pos, expr| {
//             let current = *pos;
//             *pos += expr.len() + 1;
//             Some((current, expr))
//         })
//         .filter(|(_, expr)| expr.contains('*'))
//         .map(|(pos, expr)| {
//             let paren_pos = pos + 1;
//             let mult = compute(&expr[2..]);
//             (paren_pos, mult)
//         })
//         .collect::<Vec<_>>()
//         .into_iter()
//         .rev()
//         .scan(1, |running_mult, (pos, mult)| {
//             if mult > *running_mult {
//                 *running_mult = mult;
//                 Some(Some(pos))
//             } else {
//                 Some(None)
//             }
//         })
//         .flatten()
//         .collect();
//     res.reverse();
//     res.push(expr.len());
//     res
// }

// fn left_positions(expr: &str) -> Vec<usize> {
//     let mut res: Vec<usize> = expr
//         .split('+')
//         .scan(0, |pos, expr| {
//             let current = *pos;
//             *pos += expr.len() + 1;
//             Some((current, expr))
//         })
//         .filter(|(_, expr)| expr.contains('*'))
//         .map(|(pos, expr)| {
//             let len = expr.len();
//             let paren_pos = pos + len - 1;
//             let mult = compute(&expr[..len - 2]);
//             (paren_pos, mult)
//         })
//         .scan(1, |running_mult, (pos, mult)| {
//             if mult > *running_mult {
//                 *running_mult = mult;
//                 Some(Some(pos))
//             } else {
//                 Some(None)
//             }
//         })
//         .flatten()
//         .collect();
//     res.insert(0, 0);
//     res
// }

fn compute(expr: &str) -> u64 {
    expr.split('+')
        .map(|e| {
            e.split('*')
                .map(|n| n.parse::<u64>().unwrap())
                .product::<u64>()
        })
        .sum()
}

use std::io::{self, BufWriter, Read, Write};

fn main() {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(run)
        .expect("failed to spawn worker thread")
        .join()
        .expect("worker thread panicked");
}

fn run() {
    let mut sc = Scanner::new();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        solve(&mut sc, &mut out);
    }

    out.flush().expect("failed to flush stdout");
}

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let n: usize = sc.next();
    let a = sc.next_n::<i64>(n);
    let ans: i64 = a.iter().sum();
    writeln!(out, "{}", ans).expect("failed to write output");
}

struct Scanner {
    tokens: std::vec::IntoIter<String>,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("failed to read stdin");
        let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
        Scanner {
            tokens: tokens.into_iter(),
        }
    }

    fn next<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let token = self.tokens.next().expect("ran out of input tokens");
        token
            .parse()
            .expect("token did not parse to the requested type")
    }

    fn next_n<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.next()).collect()
    }
}

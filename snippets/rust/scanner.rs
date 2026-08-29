use std::io::{self, Read};

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
}

mod io {
    #[derive(Default)]
    pub struct Scanner {
        buf: Vec<String>,
    }

    impl Scanner {
        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            if let Some(tok) = self.buf.pop() {
                return tok.parse().ok().unwrap();
            }

            let mut ln = String::new();
            std::io::stdin().read_line(&mut ln).unwrap();

            self.buf = ln.split_whitespace().rev().map(String::from).collect();

            self.next()
        }
    }
}

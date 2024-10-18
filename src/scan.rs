mod ip {
    use std::{io, str::FromStr};

    #[derive(Default, Debug)]
    pub struct Scan {
        buf: Vec<String>,
    }

    impl Scan {
        pub fn next<T: FromStr>(&mut self) -> T {
            if let Some(tok) = self.buf.pop() {
                return tok.parse().ok().expect("Parse fail");
            }

            let mut ln = String::new();
            io::stdin().read_line(&mut ln).expect("Read fail");

            self.buf = ln.split_whitespace().rev().map(String::from).collect();

            self.next()
        }
    }
}

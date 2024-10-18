mod dsu {
    use std::{num::ParseIntError, str::FromStr};

    #[derive(Clone, Debug)]
    pub struct Dsu {
        pa: Vec<u32>,
    }

    impl Dsu {
        pub fn new(len: usize) -> Self {
            Self {
                pa: (0..=len as _).collect(),
            }
        }

        pub fn rep(&mut self, u: usize) -> u32 {
            if u == self.pa[u] as _ {
                return self.pa[u];
            }

            self.pa[u] = self.rep(self.pa[u] as _);

            self.pa[u]
        }

        pub fn merge(&mut self, u: usize, v: usize) {
            let (ru, rv) = (self.rep(u), self.rep(v));

            if ru != rv {
                self.pa[ru as usize] = rv;
            }
        }

        pub fn eq_set(&mut self, u: usize, v: usize) -> bool {
            self.rep(u) == self.rep(v)
        }
    }

    impl FromStr for Dsu {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse()?))
        }
    }
}

mod up {
    #[derive(Clone, Debug)]
    pub struct BiLift {
        up: Vec<Vec<u32>>,
    }

    impl BiLift {
        pub fn new(pa: Vec<u32>) -> Self {
            let mut up = vec![pa];

            for k in 0..32 {
                up.push(up[k].iter().map(|&u| up[k][u as usize]).collect());
            }

            Self { up }
        }

        pub fn kth(&self, u: usize, k: u32) -> usize {
            if k == 0 {
                return u;
            }

            self.kth(self.up[k.trailing_zeros() as usize][u] as _, k & (k - 1))
        }

        pub fn lca(&self, mut u: usize, mut v: usize, dep: &[u32]) -> usize {
            if dep[u] < dep[v] {
                return self.lca(v, u, dep);
            }

            u = self.kth(u, dep[u] - dep[v]);

            if u == v {
                return v;
            }

            for k in (0..33).rev() {
                if self.up[k][u] == self.up[k][v] {
                    continue;
                }

                u = self.up[k][u] as _;
                v = self.up[k][v] as _;
            }

            self.up[0][u] as _
        }
    }
}

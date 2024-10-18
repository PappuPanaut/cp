mod up {
    pub struct BiLift {
        up: Vec<Vec<u32>>,
    }

    impl BiLift {
        pub fn new(pa: Vec<u32>) -> Self {
            let mut up = vec![pa];

            for k in 0..32 {
                up.push((0..up[0].len()).map(|u| up[k][up[k][u] as usize]).collect());
            }

            Self { up }
        }

        pub fn kth(&self, u: usize, k: u32) -> usize {
            if k == 0 {
                return u;
            }

            self.kth(self.up[k.trailing_zeros() as usize][u] as _, k & (k - 1))
        }
    }
}

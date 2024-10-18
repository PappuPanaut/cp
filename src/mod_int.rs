mod mi {
    use std::{
        fmt::{Display, Formatter},
        num::ParseIntError,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
        str::FromStr,
    };

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
    pub struct ModInt<const MOD: u32> {
        val: u32,
    }

    impl<const MOD: u32> ModInt<MOD> {
        pub fn new(val: u32) -> Self {
            Self { val: val % MOD }
        }

        pub fn pow(&self, exp: u32) -> Self {
            if exp == 0 {
                return ModInt::new(1);
            }

            match (exp % 2, self.pow(exp / 2)) {
                (0, p) => p * p,
                (_, p) => *self * p * p,
            }
        }

        pub fn inv(&self) -> Self {
            self.pow(MOD - 2)
        }
    }

    impl<const MOD: u32> Add for ModInt<MOD> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.val + rhs.val)
        }
    }

    impl<const MOD: u32> Neg for ModInt<MOD> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(MOD - self.val)
        }
    }

    impl<const MOD: u32> Sub for ModInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            self + -rhs
        }
    }

    impl<const MOD: u32> Mul for ModInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(((self.val as u64 * rhs.val as u64) % MOD as u64) as _)
        }
    }

    impl<const MOD: u32> Div for ModInt<MOD> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<const MOD: u32> AddAssign for ModInt<MOD> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: u32> SubAssign for ModInt<MOD> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: u32> MulAssign for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: u32> DivAssign for ModInt<MOD> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }

    impl<const MOD: u32> FromStr for ModInt<MOD> {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse()?))
        }
    }

    impl<const MOD: u32> Display for ModInt<MOD> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }
}

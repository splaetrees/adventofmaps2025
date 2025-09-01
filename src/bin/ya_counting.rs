use std::{io, ops};

const MOD: usize = 1e9 as usize + 7;

#[derive(Copy, Clone, Debug)]
struct G<const MOD: usize>(usize);

type GM = G<MOD>;

impl<const MOD: usize> ops::Add for G<MOD> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        G((self.0 + other.0) % MOD)
    }
}

impl<const MOD: usize> ops::AddAssign for G<MOD> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const MOD: usize> ops::Sub for G<MOD> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        G((self.0 + MOD - other.0) % MOD)
    }
}

impl<const MOD: usize> ops::SubAssign for G<MOD> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const MOD: usize> ops::Mul for G<MOD> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        G((self.0 * other.0) % MOD)
    }
}

impl<const MOD: usize> ops::MulAssign for G<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: usize> ops::Div for G<MOD> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let other_inv = other.pow(MOD - 2);
        self * other_inv
    }
}

impl<const MOD: usize> ops::DivAssign for G<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: usize> G<MOD> {
    fn pow(self, mut exp: usize) -> Self {
        let mut res: G<MOD> = G(1);
        let mut acc_helper: G<MOD> = self;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= acc_helper
            };
            exp /= 2;
            acc_helper *= acc_helper
        }
        res
    }
}

#[derive(Copy, Clone, Debug)]
struct Mat {
    a: GM,
    b: GM,
    c: GM,
    d: GM,
}

impl ops::Mul for Mat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Mat { a, b, c, d } = self;
        let Mat {
            a: aprime,
            b: bprime,
            c: cprime,
            d: dprime,
        } = rhs;
        Mat {
            a: a * aprime + b * cprime,
            b: a * bprime + b * dprime,
            c: c * aprime + d * cprime,
            d: c * bprime + d * dprime,
        }
    }
}

impl Mat {
    fn pow(self, mut exp: usize) -> Self {
        let mut res = Mat {
            a: G(1),
            b: G(0),
            c: G(0),
            d: G(1),
        };
        let mut acc_helper = self;
        while exp > 0 {
            if exp & 1 == 1 {
                res = res * acc_helper
            };
            exp /= 2;
            acc_helper = acc_helper * acc_helper
        }
        res
    }
}

fn mul_final(mat: Mat, init: (GM, GM)) -> GM {
    let Mat { a, b, c, d } = mat;
    a * init.0 + b * init.1 + c * init.0 + d * init.1
}

fn colourings(n: usize, m: usize) -> usize {
    let (n, m) = if n > m { (m, n) } else { (n, m) };
    let n: GM = G(n);
    let m: GM = G(m);

    match (n, m) {
        (G(..=1), m) => {
            let ans: GM = G(4) * G(3).pow(m.0 - 1);
            ans.0
        }
        (G(2), G(2)) => {
            let ans: GM = G(12) * G(2).pow(n.0 - 1);
            ans.0
        }
        (n, m) => {
            let mat = Mat {
                a: G(1),
                b: G(1),
                c: G(1),
                d: G(1),
            };
            let after_steps = mat.pow(m.0 - 3);
            ((mul_final(after_steps, (G(24), G(24))) - G(24)) + (G(12) * G(2).pow(n.0 - 1))).0
        }
    }
}

fn main() {
    let (n, m): (usize, usize) = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let split_s: Vec<&str> = s.trim().split(" ").collect();
        let [n, m] = &*split_s else {
            panic!();
        };
        let n = n.parse().unwrap();
        let m = m.parse().unwrap();
        (n, m)
    };

    println!("colourings: {}", colourings(n, m));
}

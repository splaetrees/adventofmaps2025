use itertools::Itertools;
use num::traits::One;
use rand::{self, Rng};
use std::{io, ops, str};

fn main() {
    let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();
    let s = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().trim_matches(&['[', ']']).to_owned()
    };
    for k in 1..n {
        if n % k != 0 && is_probably_prime(k, 10) {
            println!("{:3}: {}", k, decode(&s, k));
        }
    }
}

fn decode(s: &str, k: usize) -> String {
    let k_inverse = inverse(k, s.len());
    let t: Vec<u8> = (0..s.len())
        .map(|i| s.as_bytes()[i * k_inverse % s.len()])
        .collect();
    t.try_into().unwrap()
}

struct Egcd {
    g: usize,
    s: isize,
    t: isize,
}

fn egcd(a: usize, b: usize) -> Egcd {
    match (a, b) {
        (a, 0) => Egcd { g: a, s: 1, t: 0 },
        _ => {
            let Egcd {
                g,
                s: sprime,
                t: tprime,
            } = egcd(b, a % b);
            let q = a / b;
            Egcd {
                g,
                s: tprime,
                t: sprime - tprime * q as isize,
            }
        }
    }
}

fn inverse(a: usize, m: usize) -> usize {
    egcd(a, m).s.rem_euclid(m as isize) as usize
}

trait PowMod {
    fn powmod(self, exp: u32, m: Self) -> Self;
}

impl<T> PowMod for T
where
    T: Copy + One + ops::Rem<Output = Self>,
{
    fn powmod(self, mut exp: u32, m: Self) -> Self {
        let mut res = Self::one();
        let mut base = self;
        while exp > 0 {
            if exp % 2 == 1 {
                res = res * base % m;
            }
            exp = exp / 2;
            base = base * base % m;
        }
        res
    }
}

fn read_line<C: FromIterator<T>, T: str::FromStr>() -> C {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split(" ")
        .map(|sub| {
            sub.parse::<T>()
                .unwrap_or_else(|_| panic!("cannot parse into type"))
        })
        .collect()
}

fn is_probably_prime(p: usize, accuracy: usize) -> bool {
    if p <= 2 {
        return p == 2;
    }
    if p % 2 == 0 {
        return false;
    }

    let (s, d) = {
        let mut s: u32 = 0;
        let mut exp: u32 = p as u32 - 1;
        while exp % 2 == 0 {
            exp /= 2;
            s += 1;
        }
        (s, exp)
    };

    let test_prime = |a: usize| -> bool {
        if a.powmod(d, p) == 1 {
            return true;
        }
        return (0..=s)
            .map(|t| a.powmod(d * 2_u32.pow(t), p))
            .contains(&(p - 1));
    };

    let mut rng = rand::rng();
    for _ in 0..accuracy {
        let base: usize = rng.random_range(2..p as usize);
        if !test_prime(base) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod primality_test_tests {
    use super::*;

    #[test]
    fn basic_primes() {
        assert_eq!(is_probably_prime(1, 10), false);
        assert_eq!(is_probably_prime(2, 10), true);
        assert_eq!(is_probably_prime(3, 10), true);
        assert_eq!(is_probably_prime(5, 10), true);
        assert_eq!(is_probably_prime(7, 10), true);
        assert_eq!(is_probably_prime(51, 10), false);
        assert_eq!(is_probably_prime(91, 10), false);
        assert_eq!(is_probably_prime((1_usize << 23) * 7 * 17 + 1, 10), true);
    }
}

use itertools::Itertools;
use std::io;
use std::str;

struct Egcd {
    g: usize,
    s: isize,
    t: isize,
}

fn egcd(a: usize, b: usize) -> Egcd {
    match (a, b) {
        (g, 0) => Egcd { g, s: 1, t: 0 },
        (a, b) => {
            let Egcd {
                g,
                s: s_prime,
                t: t_prime,
            } = egcd(b, a % b);

            Egcd {
                g,
                s: t_prime,
                t: {
                    let q = (a / b) as isize;
                    s_prime - t_prime * q
                },
            }
        }
    }
}

fn inverse(a: usize, m: usize) -> usize {
    let m = m as isize;
    let a_inverse = (egcd(a, m as usize).s % m + m) % m;
    a_inverse as usize
}

// but 0-indexed
fn f(n: usize) -> usize {
    if n <= 1 {
        return n;
    };

    let k = n / 2;
    let m = 10usize.pow(k as u32);
    let a = 2usize.pow(k as u32);
    let b = 5usize.pow(k as u32);

    let ainv = inverse(a, b);
    let binv = inverse(b, a);
    let sol_a = a * ainv % m;
    let sol_b = b * binv % m;

    if n % 2 == 0 {
        sol_a.min(sol_b)
    } else {
        sol_a.max(sol_b)
    }
}

fn main() {
    let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();
    let ans = (0..).map(f).unique().take(n).last().unwrap();
    println!("{ans}");
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

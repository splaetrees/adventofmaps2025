use std::{io, str};

fn main() {
    let [a, b, x, y] = read_line::<Vec<usize>, _>().try_into().unwrap();
    let (x, y) = {
        let g = gcd(x, y);
        let x = x / g;
        let y = y / g;
        (x, y)
    };

    let k_a = lcm(lcm(a, x) / x, lcm(b, y) / y);
    let k_b = lcm(lcm(a, y) / y, lcm(b, x) / x);

    println!(
        "{}",
        (x * y * k_a * k_a / a / b).min(x * y * k_b * k_b / a / b)
    );
}

pub struct Egcd {
    g: usize,
    s: isize,
    t: isize,
}

pub fn egcd(a: usize, b: usize) -> Egcd {
    match (a, b) {
        (a, 0) => Egcd { g: a, s: 1, t: 0 },
        _ => {
            let Egcd {
                g,
                s: sprime,
                t: tprime,
            } = egcd(b, a % b);

            Egcd {
                g,
                s: tprime,
                t: sprime - tprime * (a / b) as isize,
            }
        }
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    egcd(a, b).g
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
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

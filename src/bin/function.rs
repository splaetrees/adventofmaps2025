use std::{io, str};

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

fn f(n: usize) -> isize {
    match n % 4 {
        0 => 2,
        1 => -2,
        2 => 1,
        _ => -3,
    }
}

fn main() {
    let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();
    println!("{}", f(n))
}

use rayon::prelude::*;
use std::{io, str};

fn main() {
    let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();
    let ans: usize = (1..=n)
        .into_par_iter()
        .map(|a| {
            (a..=n)
                .into_par_iter()
                .filter(|&b| 2 * (a + b) > n && a + 2 * b <= n)
                .count()
        })
        .sum();
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

// i forgot why i did this. but it doesn't work.
// fn main() {
//     let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();
//     let is_baka: Vec<bool> = (0..=n).fold(Vec::new(), |mut acc, i| {
//         if i == 0 {
//             acc.push(true);
//             return acc;
//         };
//         if i % 2 == 1 {
//             acc.push(false);
//             return acc;
//         };
//         let next_power = power_three_above(i);
//         if (next_power as isize) - (i as isize) < (i as isize) && acc[next_power - i - 1] {
//             acc.push(true);
//             return acc;
//         }
//         acc.push(false);
//         acc
//     });
//     let ans = is_baka.into_iter().skip(1).filter(|&x| x).count();
//     println!("{}", ans);
// }

// fn power_three_above(mut n: usize) -> usize {
//     let mut k = 0;
//     while n > 0 {
//         k += 1;
//         n /= 3;
//     }
//     (3usize).pow(k)
// }

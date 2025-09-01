use std::{io, str};
fn main() {
    let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();

    let nums: Vec<usize> = (0..n)
        .map(|_| {
            let [x] = read_line::<Vec<usize>, _>().try_into().unwrap();
            x
        })
        .collect();

    let transformed_nums: Vec<isize> = nums
        .into_iter()
        .enumerate()
        .map(|(i, val)| val as isize - i as isize)
        .collect();

    // (bsf, min_prev)
    let (ans, _) = transformed_nums
        .iter()
        .skip(1)
        .fold((0, transformed_nums[0]), |acc, &val| {
            let (bsf, min_prev) = acc;
            (bsf.max(val - min_prev), min_prev.min(val))
        });

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

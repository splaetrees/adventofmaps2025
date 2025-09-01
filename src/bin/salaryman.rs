// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use std::{io, str};

// fn main() {
//     let [n] = read_line::<Vec<usize>, _>().try_into().unwrap();
//     let mut times: Vec<[usize; 2]> = (0..n)
//         .map(|_| read_line::<Vec<usize>, _>().try_into().unwrap())
//         .collect();
//     times.sort();

//     let mut min_heap: BinaryHeap<usize> = BinaryHeap::new();
//     let mut current_time = 0;

//     let times_iter = times.into_iter().peekable();
//     while let Some([start_time, end_time]) = times_iter.next() {
//         min_heap.push(start_time.clone());
//         current_time = current_time.max(start_time.clone());
//         while let Some(earliest_end_time) = min_heap.pop() {
//             if earliest_end_time <= current_time {
//                 continue;
//             }
//         }
//     }
// }

// fn read_line<C: FromIterator<T>, T: str::FromStr>() -> C {
//     let mut s = String::new();
//     io::stdin().read_line(&mut s).unwrap();
//     s.trim()
//         .split(" ")
//         .map(|sub| {
//             sub.parse::<T>()
//                 .unwrap_or_else(|_| panic!("cannot parse into type"))
//         })
//         .collect()
// }

fn main() {
    todo!()
}

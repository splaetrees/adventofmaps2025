use itertools::Itertools;
use std::io;

const WORDS: &[&str; 32] = &[
    "MONASH",
    "ALGORITHMS",
    "PROBLEM",
    "SOLVING",
    "DIVIDEANDCONQUER",
    "BINARYSEARCH",
    "FENWICKTREE",
    "SEGMENTTREE",
    "DISJOINTSETUNION",
    "DYNAMICPROGRAMMING",
    "KNUTHSOPTIMIZATION",
    "CONVEXHULLTRICK",
    "LINEARSIEVE",
    "MODULAREXPONENTIATION",
    "DIJKSTRALGORITHM",
    "BELLMANFORDALGORITHM",
    "FLOYDWARSHALL",
    "TOPOSORT",
    "STRONGCONNECTEDCOMPONENTS",
    "BIPARTITEMATCHING",
    "HUNGARIANALGORITHM",
    "MINCOSTMAXFLOW",
    "ARTICULATIONPOINT",
    "BRIDGEOFGRAPH",
    "LCAWITHBINARYLIFTING",
    "HEAVYLIGHTDECOMPOSITION",
    "ROLLINGHASH",
    "KMPALGORITHM",
    "AHOCORASICK",
    "TRIESTRUCTURE",
    "MONOTONESTACK",
    "MONOTONEQUEUE",
];

fn find_words(grid: &Vec<Vec<char>>, words: &[&str]) -> Vec<String> {
    let diagonals: Vec<String> = (-(grid.len() as isize) + 1..grid[0].len() as isize)
        .map(|col| {
            let diagonal: String = grid
                .iter()
                .enumerate()
                .filter_map(|(i, row)| row.get(usize::try_from(col + i as isize).ok()?))
                .join("");
            diagonal
        })
        .collect();
    let rows: Vec<String> = grid.iter().map(|v| v.iter().join("")).collect();
    let cols: Vec<String> = (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).join(""))
        .collect();

    words
        .iter()
        .filter(|&&word| {
            diagonals.join("$").contains(word)
                || rows.join("$").contains(word)
                || cols.join("$").contains(word)
        })
        .map(|&s| String::from(s))
        .collect()
}

fn main() {
    let (n, _): (usize, usize) = {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut split_s = s.trim().split(" ");
        let (Some(n), Some(m), None) = (split_s.next(), split_s.next(), split_s.next()) else {
            panic!("first line should only contain two integers.");
        };
        (n.parse().unwrap(), m.parse().unwrap())
    };

    let grid: Vec<Vec<char>> = (0..n)
        .map(|_| {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            s.trim()
                .split(" ")
                .map(String::from)
                .join("")
                .chars()
                .collect()
        })
        .collect();

    println!(
        "{}",
        find_words(&grid, WORDS).into_iter().sorted().join(",")
    );
}

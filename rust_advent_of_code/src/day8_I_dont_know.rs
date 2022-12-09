
use itertools::FoldWhile::{self, Continue, Done};
use itertools::Itertools;
use memchr::memchr;

pub fn day8(path: String){
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    println!("{:?}",part1(contents.as_bytes()));
    println!("{:?}",part2(contents.as_bytes()));
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn check(b: &[u8], i: usize, j: usize, size: usize, highest: &mut u8, trees: &mut [Vec<bool>]) {
    let tree = b[i * (size + 1) + j];
    if tree > *highest {
        trees[i][j] = true;
        *highest = tree;
    }
}

pub fn part1(b: &[u8]) -> usize {
    let size = memchr(b'\n', b).unwrap();
    let mut trees = vec![vec![false; size]; size];
    for i in 1..size - 1 {
        let mut highest = b[i * (size + 1)];
        for j in 1..size - 1 {
            check(b, i, j, size, &mut highest, &mut trees);
        }
        let mut highest = b[i * (size + 1) + size - 1];
        for j in (1..size - 1).rev() {
            check(b, i, j, size, &mut highest, &mut trees);
        }
    }
    for j in 1..size - 1 {
        let mut highest = b[j];
        for i in 1..size - 1 {
            check(b, i, j, size, &mut highest, &mut trees);
        }
        let mut highest = b[(size - 1) * (size + 1) + j];
        for i in (1..size - 1).rev() {
            check(b, i, j, size, &mut highest, &mut trees);
        }
    }
    trees.iter().flatten().filter(|f| **f).count() + (size * 4 - 4)
}

fn blocking(t: u8, f: u8, acc: usize) -> FoldWhile<usize> {
    if f >= t {
        Done(acc + 1)
    } else {
        Continue(acc + 1)
    }
}

pub fn part2(b: &[u8]) -> usize {
    let size = memchr(b'\n', b).unwrap();
    let mut hiscore = 0;
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let t = b[i * (size + 1) + j];
            hiscore = hiscore.max(
                (0..j)
                    .rev()
                    .map(|k| b[i * (size + 1) + k])
                    .fold_while(0, |acc, f| blocking(t, f, acc))
                    .into_inner()
                    * (j + 1..size)
                        .map(|k| b[i * (size + 1) + k])
                        .fold_while(0, |acc, f| blocking(t, f, acc))
                        .into_inner()
                    * (0..i)
                        .rev()
                        .map(|k| b[k * (size + 1) + j])
                        .fold_while(0, |acc, f| blocking(t, f, acc))
                        .into_inner()
                    * (i + 1..size)
                        .map(|k| b[k * (size + 1) + j])
                        .fold_while(0, |acc, f| blocking(t, f, acc))
                        .into_inner(),
            );
        }
    }
    hiscore
}

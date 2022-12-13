
use memchr::memchr;
use std::{collections::VecDeque, io::Read};

fn bfs(b: &mut [u8]) -> usize {
    let w = memchr(b'\n', b).unwrap() + 1;
    let start = memchr(b'S', b).unwrap();
    let end = memchr(b'z', b).unwrap();

    b[start] = 255;
    b[end] = b'z';

    let mut q = VecDeque::from([(start, 0, b'a')]);

    while let Some((pos, steps, height)) = q.pop_front() {
        for next_pos in [pos + 1, pos + w, pos.wrapping_sub(1), pos.wrapping_sub(w)] {
            let next_height = *b.get(next_pos).unwrap_or(&255);
            if next_height == b'\n' || (next_height - 1) > height {
                continue;
            } else if next_pos == end {
                return steps + 1;
            }
            b[next_pos] = 255;
            q.push_back((next_pos, steps + 1, next_height));
        }
    }

    panic!()
}

fn reverse_bfs(b: &mut [u8]) -> usize {
    let w = memchr(b'\n', b).unwrap() + 1;
    let end = memchr(b'S', b).unwrap();
    let start = memchr(b'E', b).unwrap();

    b[start] = 255;
    b[end] = b'a';

    let mut q = VecDeque::from([(start, 0, b'z')]);

    while let Some((pos, steps, height)) = q.pop_front() {
        for next_pos in [pos + 1, pos + w, pos.wrapping_sub(1), pos.wrapping_sub(w)] {
            let next_height = *b.get(next_pos).unwrap_or(&0);
            if next_height == b'\n' || (next_height + 1) < height {
                continue;
            } else if b[next_pos] == b'a' {
                return steps + 1;
            }
            b[next_pos] = 255;
            q.push_back((next_pos, steps + 1, next_height));
        }
    }

    panic!()
}

fn part1(b: &[u8]) -> usize {
    bfs(&mut b.to_vec())
}

fn part2(b: &[u8]) -> usize {
    reverse_bfs(&mut b.to_vec())
}

pub fn day12(path: String){
    let start = std::time::Instant::now();
    let other = include_bytes!("/home/matt/advent_of_code_2022/files/day12.txt");
    println!("{}",part1(other));
    println!("{}",part2(other));
    println!("Time elapsed is: {:?}", start.elapsed());
}


use std::collections::VecDeque;

struct Rope {
    // the two-dimensional grid representing the rope
    grid: Vec<Vec<char>>,
    // the position of the head of the rope
    head: (i32, i32),
    // the position of the tail of the rope
    tail: (i32, i32),
}

impl Rope {
    fn new(grid: Vec<Vec<char>>, head: (i32, i32), tail: (i32, i32)) -> Self {
        Self { grid, head, tail }
    }

    fn move_head(&mut self, direction: char) {
        // update the position of the head based on the direction
        match direction {
            'U' => self.head.1 -= 1,
            'D' => self.head.1 += 1,
            'L' => self.head.0 -= 1,
            'R' => self.head.0 += 1,
            _ => {}
        }

        // update the position of the tail based on the new position of the head
        self.update_tail();
    }

    fn update_tail(&mut self) {
        // if the head and tail are in the same row or column, move the tail one step
        // in the same direction as the head
        if self.head.0 == self.tail.0 {
            if self.head.1 > self.tail.1 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
        } else if self.head.1 == self.tail.1 {
            if self.head.0 > self.tail.0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }
        } else {
            // otherwise, move the tail one step diagonally to keep up with the head
            if self.head.0 > self.tail.0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }
            if self.head.1 > self.tail.1 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
        }
    }
}

pub fn day9() {
    // initialize the rope with a grid and the starting positions of the head and tail
    // let mut rope = Rope::new(
    //     vec![
    //         vec!['.', '.', '.', '.', '.', '.', '.'],
    //         vec!['.', '.', '.', '.', '.', '.', '.'],
    //         vec!['.', '.', '.', '.', '.', '.', '.'],
    //         vec!['.', '.', '.', '.', '.', '.', '.']]);
    }
           


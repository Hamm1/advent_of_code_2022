
#[derive(Debug)]
struct Rope {
    grid: Vec<Vec<char>>,
    head: (i32, i32),
    tail: (i32, i32),
    scores: std::collections::HashMap<(i32,i32), i32>
}

impl Rope {
    fn new(grid: Vec<Vec<char>>, head: (i32, i32), tail: (i32, i32), scores: std::collections::HashMap<(i32,i32), i32>) -> Self {
        Self { grid, head, tail, scores }
    }

    fn move_head(&mut self, direction: char, spaces: i32) {
        //println!("{direction},{spaces}");
        //println!("Starting Head Position {:?}",self.head);
        for _i in 0..spaces{
            match direction {
                'U' => self.head.0 -= 1,
                'D' => self.head.0 += 1,
                'L' => self.head.1 -= 1,
                'R' => self.head.1 += 1,
                _ => {}
            }
            //println!("Head: {:?}",self.head);
            self.update_tail();
            //println!("Tail: {:?}",self.tail);
            self.scores.insert(self.tail, 0);
        }
    }

    fn update_tail(&mut self) {
        if self.head.0 == self.tail.0{
            if self.head.1 > self.tail.1+1{
                self.tail.1 += 1;
            } else if self.head.1 > self.tail.1{
                self.tail.0 = self.head.0;
                self.tail.1 = self.head.1-1;
            }
        }
        if self.head.0 < self.tail.0{
            if self.head.0 > self.tail.0+1{

            } else if self.head.0 < self.tail.0-1 {
                self.tail.0 = self.head.0+1; 
                self.tail.1 = self.head.1
            } else if self.head.0 < self.tail.0 && self.head.1 > self.tail.1+1 {
                self.tail.0 = self.head.0;
                self.tail.1 = self.head.1-1
            }
        } else if self.head.0 > self.tail.0{
            if self.head.1 > self.tail.1+1{
                self.tail.0 = self.head.0;
                self.tail.1 = self.head.1-1;
            } 
        }
        if self.head.1 < self.tail.1-1{
            self.tail.0 = self.head.0;
            self.tail.1 = self.head.1+1;
        } else if self.head.0 > self.tail.0+1{
            self.tail.1 = self.head.1; 
            self.tail.0 = self.head.0-1
        }
    }
}

pub fn day9(path: String) {
    let start = std::time::Instant::now();
    let scores: std::collections::HashMap<(i32,i32),i32> = std::collections::HashMap::new();
    let mut grid: Vec<Vec<char>> = vec![];
    for _amount_of_vectors in 0..5{
        grid.push(vec![]);  
    }
    for height in 0..5{
        let thing: char = '.';
        for _length in 0..6{
            grid[height].push(thing);
        }
    }
    let mut rope = Rope::new(grid,(4,0),(4,0),scores);
    let contents = std::fs::read_to_string(path).unwrap();
    for line in contents.lines(){
        let instructions: Vec<&str> = line.split(" ").collect();
        Rope::move_head(&mut rope, instructions[0].parse::<char>().unwrap(), instructions[1].parse::<i32>().unwrap())
    }
    println!("End Head Position: {:?}",rope.head);
    println!("End Tail Position: {:?}",rope.tail);
    println!("Day 9 Part 1: Total {:?}",rope.scores.len());
    println!("Time elapsed is: {:?}", start.elapsed());
}
           
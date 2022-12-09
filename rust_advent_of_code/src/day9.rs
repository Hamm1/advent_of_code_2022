
#[derive(Debug)]
struct Rope {
    grid: Vec<Vec<char>>,
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    fn new(grid: Vec<Vec<char>>, head: (i32, i32), tail: (i32, i32)) -> Self {
        Self { grid, head, tail }
    }

    fn move_head(&mut self, direction: char, spaces: i32) {
        println!("{:?}",self.head);
        for _i in 0..spaces{
            match direction {
                'U' => self.head.0 -= 1,
                'D' => self.head.0 += 1,
                'L' => self.head.1 -= 1,
                'R' => self.head.1 += 1,
                _ => {}
            }
        
            println!("{direction},{spaces}");
            println!("{:?}",self.head);
            self.update_tail();
        }
    }

    fn update_tail(&mut self) {
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

pub fn day9(path: String) {
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
    let mut rope = Rope::new(grid,(4,0),(4,0));
    let contents = std::fs::read_to_string(path).unwrap();
    for line in contents.lines(){
        let instructions: Vec<&str> = line.split(" ").collect();
        Rope::move_head(&mut rope, instructions[0].parse::<char>().unwrap(), instructions[1].parse::<i32>().unwrap())
    }
    println!("{:?}",rope.head);
    println!("{:?}",rope.tail);
}
           



#[derive(Debug)]
struct Hiker {
    grid: Vec<Vec<char>>,
    current_position: (usize, usize),
    previous_position: (usize, usize),
    priority: Vec<char>,
    current_priority: usize,
    visited: Vec<(usize, usize)>,
    status: String
}
impl Hiker {
    fn new(grid: Vec<Vec<char>>, current_position: (usize, usize), previous_position: (usize, usize), priority: Vec<char>, current_priority: usize, visited: Vec<(usize, usize)>, status: String ) -> Self{
        Self{ grid, current_position, previous_position, priority, current_priority, visited, status }
    }
    fn move_hiker(&mut self){
        let mut check = false;
        let current_letter = self.grid[self.current_position.0][self.current_position.1];
        match self.priority.iter().position(|&x| x == current_letter.clone()).ok_or("err"){
            Ok(d) => self.current_priority = d + 1,
            Err(_) => self.current_priority = 0
        };
        let mut z = self.priority[self.current_priority];
        if self.priority[self.current_priority] == 'E'{
            self.status = "Complete".to_string();
        } else {
            println!("{:?}", z);
            check = self.move_hiker_check(check, z);
            if check == false{
                z = self.priority[self.current_priority-1];
                println!("{:?}", z);
                check = self.move_hiker_check(check, z);
            }
            println!("{:?}",self.current_position)
        }
    }
    fn move_hiker_check(&mut self, mut check: bool, z: char) -> bool{
        if self.current_position.0+1 > 0 &&  self.current_position.0+1 < self.grid.len(){
            if self.grid[self.current_position.0+1][self.current_position.1] == z && check != true && (self.current_position.0+1,self.current_position.1) != self.previous_position{
                self.previous_position.0 = self.current_position.0;
                self.previous_position.1 = self.current_position.1;
                println!("Previous {},{}",self.previous_position.0, self.previous_position.1);
                println!("Move down to {},{}",self.current_position.0+1,self.current_position.1);
                self.current_position.0 = self.current_position.0+1;
                check = true
            }
        }
        if self.current_position.0 != 0 {
            if (self.current_position.0-1).to_string().parse::<i32>().unwrap() >= 0 &&  (self.current_position.0-1) < self.grid.len(){
                if self.grid[self.current_position.0-1][self.current_position.1] == z && check != true && (self.current_position.0-1,self.current_position.1) != self.previous_position{
                    self.previous_position.0 = self.current_position.0;
                    self.previous_position.1 = self.current_position.1;
                    println!("Previous {},{}",self.previous_position.0,self.previous_position.1);
                    println!("Move up to {},{}",self.current_position.0-1,self.current_position.1);
                    self.current_position.0 = self.current_position.0-1;
                    check = true
                }
            }
        }
        if self.current_position.1+1 > 0 &&  self.current_position.1+1 < self.grid[0].len(){
            if self.grid[self.current_position.0][self.current_position.1+1] == z && check != true && (self.current_position.0,self.current_position.1+1) != self.previous_position{
                self.previous_position.0 = self.current_position.0;
                self.previous_position.1 = self.current_position.1;
                println!("Previous {},{}",self.previous_position.0,self.previous_position.1);
                println!("Move right to {},{}",self.current_position.0,self.current_position.1+1);
                self.current_position.1 = self.current_position.1+1;
                check = true
            }
        }
        if self.current_position.1 != 0 {
            if self.current_position.1 > 0 &&  self.current_position.1 < self.grid[0].len() {
                if self.grid[self.current_position.0][self.current_position.1-1] == z && check != true && (self.current_position.0,self.current_position.1-1) != self.previous_position{
                    self.previous_position.0 = self.current_position.0;
                    self.previous_position.1 = self.current_position.1;
                    println!("Previous {},{}",self.previous_position.0,self.previous_position.1);
                    println!("Move left to {},{}",self.current_position.0,self.current_position.1-1);
                    self.current_position.1 = self.current_position.1-1;
                    check = true
                }
            }
        }
        return check
    }
}
pub fn day12(path: String){
    let mut grid: Vec<Vec<char>> = vec![];
    let visited: Vec<(usize, usize)> = vec![];
    let mut priority: Vec<char> = vec!['S'];
    priority.extend(('a'..='z').into_iter().collect::<Vec<char>>());
    priority.push('E');
    let contents = std::fs::read_to_string(path).unwrap();
    for line in contents.lines(){
        let collection: Vec<char> = line.chars().collect();
        grid.push(collection);
    }
    let mut hiker = Hiker::new(grid, (0,0), (0,0), priority, 0, visited, "not_complete".to_string());
    println!("{:?}",hiker);
    let mut i = 0;
    while hiker.status != "Complete"{
        Hiker::move_hiker(&mut hiker);
        i = i + 1
    }
    println!("{}",i);
}

// let eth = vec![
//         vec![0,1,2,17,16,15,14,13],
//         vec![1,2,3,18,25,24,24,12],
//         vec![1,3,3,19,26,27,24,11],
//         vec![1,3,3,20,21,22,23,10],
//         vec![1,2,4,5,6,7,8,9],
        
//     ];
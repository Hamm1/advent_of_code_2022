
pub fn day10(path: String){
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    let mut x = 1;
    let mut cycle = 0;
    let mut overall = 0;
    let mut grid: Vec<Vec<&str>> = vec![];
    let mut message: String = "".to_string();
    for _amount_of_vectors in 0..5{
        grid.push(vec![]);  
    }
    for height in 0..5{
        let thing: &str = ".";
        for _length in 0..40{
            grid[height].push(thing);
        }
    }
    'outer:for line in contents.lines(){
        if line == "noop"{
            cycle = cycle + 1;
            if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220{
                //break 'outer
                overall = overall + (cycle * x);
            }
            //println!("Cycle {cycle} Value {x}");
        } else {
            let new_line = line.replace("addx ", "");
            for _i in 0..2{
                cycle = cycle + 1;
                if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220{
                    //break 'outer
                    overall = overall + (cycle * x);
                }
                if _i == 0 {
                    //println!("Cycle {cycle} Value {x}");
                }
            }
            x = x + new_line.parse::<i32>().unwrap();
            println!("Cycle {cycle} Value {x}");
            // // ! Part 2
            // if cycle <= 40{
            //     if cycle == 1 || cycle == 2{
            //         let got = std::mem::replace(&mut grid[0][0], "#");
            //     }
            //     let got = std::mem::replace(&mut grid[0][x.to_string().parse::<usize>().unwrap()], "#");
            //     //break 'outer
                
            // }
            // if cycle > 40 && cycle < 80{
            //     let got = std::mem::replace(&mut grid[1][x.to_string().parse::<usize>().unwrap()], "#");
            // }
            // if cycle > 80 && cycle < 120{
            //     let got = std::mem::replace(&mut grid[2][x.to_string().parse::<usize>().unwrap()], "#");
            // }

        }
    }
    println!("{}",x);
    println!("{}",cycle);
    println!("Day 10 Part 1: {}",overall);
    println!("Time elapsed is: {:?}", start.elapsed());
    println!("{:?}",grid);
    println!("{:?}", message)
}
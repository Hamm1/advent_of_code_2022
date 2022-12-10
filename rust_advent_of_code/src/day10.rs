
pub fn day10(path: String){
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    let mut x = 1;
    let mut cycle = 0;
    let mut overall = 0;
    let mut grid: Vec<Vec<&str>> = vec![];
    for _amount_of_vectors in 0..6{
        grid.push(vec![]);  
    }
    for height in 0..6{
        let thing: &str = " ";
        for _length in 0..40{
            grid[height].push(thing);
        }
    }
    for line in contents.lines(){
        if line == "noop"{
            cycle = cycle + 1;
            overall = part1(cycle, x, overall);
            grid = part2_render(grid.clone(), cycle, x);
        } else {
            let new_line = line.replace("addx ", "");
            for _i in 0..2{
                cycle = cycle + 1;
                overall = part1(cycle, x, overall);
                if _i == 1{
                    x = x + new_line.parse::<i32>().unwrap();
                }
                grid = part2_render(grid.clone(), cycle, x);
            }
        }
    }
    println!("Day 10 Part 1: {}",overall);
    println!("Day 10 Part 2:");
    for g in grid{
        println!("{:?}",g.concat());
    }
    println!("Time elapsed is: {:?}", start.elapsed());
}
fn part1(cycle: i32, x: i32, mut overall: i32) -> i32{
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220{
        overall = overall + (cycle * x);
    }
    return overall
}
fn part2_render(mut grid: Vec<Vec<&str>>,mut cycle: i32,x: i32) ->Vec<Vec<&str>>{
    let mut m = 0;
    let new_cycle: usize = cycle.to_string().parse::<usize>().unwrap();
    for i in 0..grid.len(){
        if new_cycle > 40 * i && new_cycle < 40 * (i+1){
            cycle = cycle - (40 * i).to_string().parse::<i32>().unwrap();
            let _got = std::mem::replace(&mut grid[i][0], "#");
            m = i;
        }
    }
    if cycle == x || cycle == x+1 || cycle == x-1{
        let _ = std::mem::replace(&mut grid[m][cycle.to_string().parse::<usize>().unwrap()], "#");
    }
    return grid
}
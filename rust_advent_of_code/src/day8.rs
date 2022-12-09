
pub fn day8(path: String){
    let start = std::time::Instant::now();
    let mut grid: Vec<Vec<i32>> = vec![];
    let contents = std::fs::read_to_string(path).unwrap();
    let mut i = 0;
    for line in contents.lines(){
        grid.push(vec![]);
        let new_line: Vec<char> = line.chars().collect();
        for new in new_line{
            let thing = new.to_string().parse::<i32>().unwrap();
            grid[i].push(thing);
        }
        i = i + 1;
    }
    //println!("{:?}",grid);
    let mut scores: std::collections::HashMap<String,i32> = std::collections::HashMap::new();
    for i in 0..grid.len(){
        let grid_left_to_right = 0;
        let grid_right_to_left = grid[i].len()-1;
        let grid_top_to_bottom = 0;
        let grid_bottom_to_top = grid.len()-1;
        if i == 0 {
            for m in 0..grid.len(){
                scores.insert(String::from(0.to_string() + "," + (m).to_string().as_str()), 0);
                scores.insert(String::from((grid[0].len()-1).to_string() + "," + (m).to_string().as_str()), 0);
                scores.insert(String::from(m.to_string() + "," + 0.to_string().as_str()), 0);
                scores.insert(String::from((m).to_string().to_string() + "," + (grid.len()-1).to_string().as_str()), 0);
                
            }
            let mut m = 0;
            while m <= grid[0].len()-1{
                let mut highest;
                highest = grid[grid_top_to_bottom][m];
                let mut highest_detected = false;
                for grid_top_to_bottom in 0..grid.len()-1{
                    if grid[grid_top_to_bottom][m] < grid[grid_top_to_bottom+1][m] && highest_detected == false{
                        scores.insert(String::from((grid_top_to_bottom).to_string().to_string() + "," + m.to_string().as_str()), 0);
                    }
                    if highest < grid[grid_top_to_bottom+1][m]{
                        highest = grid[grid_top_to_bottom+1][m];
                        highest_detected = true;
                        scores.insert(String::from((grid_top_to_bottom+1).to_string() + "," + m.to_string().as_str()), 0);
                    }
                    if grid[grid_top_to_bottom][m] >= grid[grid_top_to_bottom+1][m]{
                        highest_detected = true;
                    }
                    if grid_top_to_bottom == grid[0].len().to_string().parse::<usize>().unwrap(){
                        scores.insert(String::from((grid_top_to_bottom+1).to_string() + "," + m.to_string().as_str()), 0);
                    }
                }

                highest = grid[grid_bottom_to_top][m];
                let mut highest_detected = false;
                for grid_bottom_to_top in (1..grid.len()).rev(){
                    if grid[grid_bottom_to_top][m] < grid[grid_bottom_to_top-1][m] && highest_detected == false{
                        scores.insert(String::from((grid_bottom_to_top).to_string().to_string() + "," + m.to_string().as_str()), 0);
                    }
                    if highest < grid[grid_bottom_to_top-1][m]{
                        highest = grid[grid_bottom_to_top-1][m];
                        highest_detected = true;
                        scores.insert(String::from((grid_bottom_to_top-1).to_string() + "," + m.to_string().as_str()), 0);
                    }
                    if grid[grid_bottom_to_top][m] >= grid[grid_bottom_to_top-1][m]{
                        highest_detected = true;
                    }
                    if grid_bottom_to_top == grid[0].len().to_string().parse::<usize>().unwrap(){
                        scores.insert(String::from((grid_bottom_to_top-1).to_string() + "," + m.to_string().as_str()), 0);
                    }
                }
                m = m + 1;
            }
        }
        let mut highest;
        highest = grid[i][grid_left_to_right];
        let mut highest_detected = false;
        for grid_left_to_right in 0..grid[i].len()-1{
            if grid[i][grid_left_to_right] < grid[i][grid_left_to_right+1] && highest_detected == false{
                scores.insert(String::from(i.to_string() + "," + (grid_left_to_right).to_string().as_str()), 0);
            }
            if highest < grid[i][grid_left_to_right+1]{
                highest = grid[i][grid_left_to_right+1];
                highest_detected = true;
                scores.insert(String::from(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str()), 0);
            }
            if grid[i][grid_left_to_right] >= grid[i][grid_left_to_right+1]{
                highest_detected = true;
            }
            if grid_left_to_right == 0{
                scores.insert(String::from(i.to_string() + "," + (grid_left_to_right).to_string().as_str()), 0);
            }
        }
        highest = grid[i][grid_right_to_left];
        let mut highest_detected = false;
        for grid_right_to_left in (1..grid[i].len()).rev(){
            if grid[i][grid_right_to_left] < grid[i][grid_right_to_left-1] && highest_detected == false{
                scores.insert(String::from(i.to_string() + "," + (grid_right_to_left).to_string().as_str()), 0);
            }
            if highest < grid[i][grid_right_to_left-1]{
                highest = grid[i][grid_right_to_left-1];
                highest_detected = true;
                scores.insert(String::from(i.to_string() + "," + (grid_right_to_left-1).to_string().as_str()), 0);
            }
            if grid[i][grid_right_to_left] >= grid[i][grid_right_to_left-1]{
                highest_detected = true;
            }
            if grid_right_to_left+1 == grid[i].len().to_string().parse::<usize>().unwrap(){
                scores.insert(String::from(i.to_string() + "," + (grid_right_to_left).to_string().as_str()), 0);
            }
        }
    }
    println!("Day 8 part 1: Total {:?}",scores.len());
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
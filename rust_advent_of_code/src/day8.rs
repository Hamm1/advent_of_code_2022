
pub fn day8(path: String){
    //let start = std::time::Instant::now();
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
    let mut total: i32 = 0;
    //println!("{:?}",grid);
    let mut seen: Vec<String> = vec![];
    let mut scores: std::collections::HashMap<String,i32> = std::collections::HashMap::new();
    for i in 0..grid.len(){
        let mut grid_left_to_right = 0;
        let mut grid_right_to_left = grid[i].len()-1;
        let mut grid_top_to_bottom = 0;
        let mut grid_bottom_to_top = grid.len()-1;
        if i == 0 {
            for m in 0..grid.len(){
                // scores.insert(String::from(0.to_string() + "," + (m).to_string().as_str()), 0);
                // scores.insert(String::from((grid[0].len()-1).to_string() + "," + (m).to_string().as_str()), 0);
                // scores.insert(String::from(m.to_string() + "," + 0.to_string().as_str()), 0);
                // scores.insert(String::from((m).to_string().to_string() + "," + (grid.len()-1).to_string().as_str()), 0);
                
            }
            let mut m = 0;
        //     while m <= grid[0].len()-1{
        //         grid_top_to_bottom = 0;
        //         grid_bottom_to_top = grid.len()-1;
        //         let mut highest = 0;
        //         while grid[grid_top_to_bottom][m] <= grid[grid_top_to_bottom+1][m] || highest >= grid[grid_top_to_bottom][m]{
        //             if highest < grid[grid_top_to_bottom][m]{
        //                 highest = grid[grid_top_to_bottom+1][m];
        //                 scores.insert(String::from((grid_top_to_bottom+1).to_string() + "," + m.to_string().as_str()), 0);
        //             } else if grid[grid_top_to_bottom][m] < grid[grid_top_to_bottom+1][m]{
        //                 scores.insert(String::from((grid_top_to_bottom+1).to_string() + "," + m.to_string().as_str()), 0);
        //             }
        //             // seen.push((grid_left_to_right+1).to_string() + "," + i.to_string().as_str());
        //             // total = total + 1;
        //             // println!("{:?}",(grid_top_to_bottom+1).to_string() + "," + m.to_string().as_str());
        //             grid_top_to_bottom = grid_top_to_bottom + 1;
        //             //m = m + 1;
        //             if grid_top_to_bottom > grid.len()-1{
        //                 highest = 0;
        //                 break
        //             }
        //         }
                    
        //         highest = 0;
        //         while grid[grid_bottom_to_top][m] <= grid[grid_bottom_to_top-1][m] || highest >= grid[grid_bottom_to_top][m]{
        //             if highest < grid[grid_bottom_to_top][m]{
        //                 highest = grid[grid_bottom_to_top-1][m];
        //                 scores.insert(String::from((grid_bottom_to_top-1).to_string() + "," + m.to_string().as_str()), 0);
        //             } else if grid[grid_bottom_to_top][m] < grid[grid_bottom_to_top-1][m]{
        //                 scores.insert(String::from((grid_bottom_to_top-1).to_string() + "," + m.to_string().as_str()), 0);
        //             }
        //             // seen.push((grid_bottom_to_top-1).to_string() + "," + i.to_string().as_str());
        //             total = total + 1;
        //             //println!("{:?}",(grid_bottom_to_top-1).to_string() + "," + m.to_string().as_str());
        //             grid_bottom_to_top = grid_bottom_to_top - 1;
        //             //m = m + 1;
        //             if grid_bottom_to_top == 0{
        //                 break
        //             }
                    
        //         }
                let mut highest = 0;
                highest = grid[grid_bottom_to_top][m];
                let mut highest_detected = false;
                for grid_bottom_to_top in (1..grid.len()).rev(){
                    if grid[grid.len()-1][grid_bottom_to_top] < grid[grid.len()-1][grid_bottom_to_top-1] && highest_detected == false{
                        scores.insert(String::from(i.to_string() + "," + (grid_bottom_to_top).to_string().as_str()), 0);
                    }
                    if highest < grid[grid.len()-1][grid_bottom_to_top-1]{
                        highest = grid[grid.len()-1][grid_bottom_to_top-1];
                        highest_detected = true;
                        println!("New Highest: {}",highest);
                        scores.insert(String::from(i.to_string() + "," + (grid_bottom_to_top-1).to_string().as_str()), 0);
                    }
                    if grid[grid_bottom_to_top][m] >= grid[grid_bottom_to_top-1][m]{
                        highest_detected = true;
                    }
                    if grid_bottom_to_top+1 == grid[m].len().to_string().parse::<usize>().unwrap(){
                        scores.insert(String::from(m.to_string() + "," + (grid_bottom_to_top).to_string().as_str()), 0);
                    }
                }
                m = m + 1;
        //     }
        }
        let mut highest = 0;
        // while grid[i][grid_left_to_right] <= grid[i][grid_left_to_right+1] || highest >= grid[i][grid_left_to_right]{
        //     if highest < grid[i][grid_left_to_right]{
        //         highest = grid[i][grid_left_to_right+1];
        //         scores.insert(String::from(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str()), 0);
        //     } else if grid[i][grid_left_to_right] < grid[i][grid_left_to_right+1]{
        //         scores.insert(String::from(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str()), 0);
        //     }
        //     // seen.push(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str());
        //     total = total + 1;
        //     // println!("{:?}",i.to_string() + "," + (grid_left_to_right+1).to_string().as_str());
        //     grid_left_to_right = grid_left_to_right + 1;
        //     if grid_left_to_right > grid.len()-1{
        //         highest = 0;
        //         break
                
        //     }
        // }
        // highest = grid[i][grid_left_to_right];
        // let mut highest_detected = false;
        // for grid_left_to_right in (0..grid[i].len()-1){
        //     if grid[i][grid_left_to_right] < grid[i][grid_left_to_right+1] && highest_detected == false{
        //         scores.insert(String::from(i.to_string() + "," + (grid_left_to_right).to_string().as_str()), 0);
        //     }
        //     if highest < grid[i][grid_left_to_right+1]{
        //         highest = grid[i][grid_left_to_right+1];
        //         highest_detected = true;
        //         println!("New Highest: {}",highest);
        //         scores.insert(String::from(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str()), 0);
        //     }
        //     if grid[i][grid_left_to_right] >= grid[i][grid_left_to_right+1]{
        //         highest_detected = true;
        //     }
        //     if grid_left_to_right == 0{
        //         scores.insert(String::from(i.to_string() + "," + (grid_left_to_right).to_string().as_str()), 0);
        //     }
        // }
        // highest = grid[i][grid_right_to_left];
        // while grid[i][grid_right_to_left] <= grid[i][grid_right_to_left-1] || highest >= grid[i][grid_right_to_left-1]{
        //     println!("Current Highest: {highest}");
        //     if highest < grid[i][grid_right_to_left]{
        //         highest = grid[i][grid_right_to_left-1];
        //         println!("New Highest: {highest}");
        //         //scores.insert(String::from(i.to_string() + "," + (grid_right_to_left).to_string().as_str()), 0);
        //         scores.insert(String::from(i.to_string() + "," + (grid_right_to_left-1).to_string().as_str()), 0);
        //     } else if grid[i][grid_right_to_left] < grid[i][grid_right_to_left-1]{
        //         //println!("{}","hey");
        //         // println!("{:?}",i.to_string() + "," + (grid_right_to_left-1).to_string().as_str());
        //         scores.insert(String::from(i.to_string() + "," + (grid_right_to_left).to_string().as_str()), 0);
        //     }
        //     // seen.push(i.to_string() + "," + (grid_right_to_left-1).to_string().as_str());
        //     //println!("{:?}",i.to_string() + "," + (grid_right_to_left-1).to_string().as_str());
        //     total = total + 1;
        //     grid_right_to_left = grid_right_to_left - 1;
        //     if grid_right_to_left == 0{
        //         break
        //     }
        // }
        // highest = grid[i][grid_right_to_left];
        // let mut highest_detected = false;
        // for grid_right_to_left in (1..grid[i].len()).rev(){
        //     if grid[i][grid_right_to_left] < grid[i][grid_right_to_left-1] && highest_detected == false{
        //         scores.insert(String::from(i.to_string() + "," + (grid_right_to_left).to_string().as_str()), 0);
        //     }
        //     if highest < grid[i][grid_right_to_left-1]{
        //         highest = grid[i][grid_right_to_left-1];
        //         highest_detected = true;
        //         println!("New Highest: {}",highest);
        //         scores.insert(String::from(i.to_string() + "," + (grid_right_to_left-1).to_string().as_str()), 0);
        //     }
        //     if grid[i][grid_right_to_left] >= grid[i][grid_right_to_left-1]{
        //         highest_detected = true;
        //     }
        //     if grid_right_to_left+1 == grid[i].len().to_string().parse::<usize>().unwrap(){
        //         scores.insert(String::from(i.to_string() + "," + (grid_right_to_left).to_string().as_str()), 0);
        //     }
        // }
        
        
    }
    //println!("{}",total);
    //println!("{:?}",seen);
    println!("{:?}",scores);
    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);
}
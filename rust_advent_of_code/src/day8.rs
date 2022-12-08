
pub fn day8(path: String){
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
    println!("{:?}",grid);
    let mut seen: Vec<String> = vec![];
    let mut scores: std::collections::HashMap<String,i32> = std::collections::HashMap::new();
    for i in 0..grid.len(){
        let mut grid_left_to_right = 0;
        let mut grid_right_to_left = grid[i].len()-1;
        let mut grid_top_to_bottom = 0;
        let mut grid_bottom_to_top = grid.len()-1;
        if i == 0 {
            for m in 0..grid.len(){
                scores.insert(String::from(0.to_string() + "," + (m).to_string().as_str()), 0);
                println!("{}",String::from(0.to_string() + "," + (m).to_string().as_str()));
                scores.insert(String::from((grid[0].len()-1).to_string() + "," + (m).to_string().as_str()), 0);
                println!("{}",String::from((grid[0].len()-1).to_string() + "," + (m).to_string().as_str()));
                scores.insert(String::from(m.to_string() + "," + 0.to_string().as_str()), 0);
                println!("{}",String::from(m.to_string() + "," + 0.to_string().as_str()));
                scores.insert(String::from((m).to_string().to_string() + "," + (grid.len()-1).to_string().as_str()), 0);
                println!("{}",String::from((m).to_string().to_string() + "," + (grid.len()-1).to_string().as_str()));
                
            }
        }
        while grid[i][grid_left_to_right] < grid[i][grid_left_to_right+1]{
            scores.insert(String::from(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str()), 0);
            // seen.push(i.to_string() + "," + (grid_left_to_right+1).to_string().as_str());
            total = total + 1;
            grid_left_to_right = grid_left_to_right + 1
        }
        while grid[i][grid_right_to_left] > grid[i][grid_right_to_left-1]{
            scores.insert(String::from(i.to_string() + "," + (grid_right_to_left-1).to_string().as_str()), 0);
            // seen.push(i.to_string() + "," + (grid_right_to_left-1).to_string().as_str());
            total = total + 1;
            grid_right_to_left = grid_right_to_left - 1
        }
        while grid[grid_top_to_bottom][i] < grid[grid_top_to_bottom+1][i]{
            scores.insert(String::from((grid_left_to_right+1).to_string() + "," + i.to_string().as_str()), 0);
            // seen.push((grid_left_to_right+1).to_string() + "," + i.to_string().as_str());
            total = total + 1;
            grid_top_to_bottom = grid_top_to_bottom + 1;
        }
        while grid[grid_bottom_to_top][i] > grid[grid_bottom_to_top][i]{
            scores.insert(String::from((grid_bottom_to_top-1).to_string() + "," + i.to_string().as_str()), 0);
            // seen.push((grid_bottom_to_top-1).to_string() + "," + i.to_string().as_str());
            total = total + 1;
            grid_bottom_to_top = grid_bottom_to_top - 1;
        }
        
    }
    println!("{}",total);
    //println!("{:?}",seen);
    println!("{:?}",scores.len());
}
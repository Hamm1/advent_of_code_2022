
pub fn day7(path: String){
    let start = std::time::Instant::now();
    let mut scores = std::collections::HashMap::new();
    //let mut scores2: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let contents = std::fs::read_to_string(path).unwrap();
    let mut current_dir: String = "".to_string();
    for line in contents.lines(){
        if line.contains("dir"){
            let something = current_dir.to_owned() + " " + line.replace("dir ", "").as_str();
            scores.insert(String::from(something.clone()),"0");
        }
        if line.contains("$ cd") && !line.contains("$ cd .."){
            if current_dir.is_empty(){
                current_dir = line.replace("$ cd ", "");
            } else {
                current_dir = current_dir.to_owned() + " " + line.replace("$ cd ", "").as_str();
            }
            scores.insert(String::from(current_dir.clone()),"0");
        }
        if !line.contains("dir") && !line.contains("$"){
            let file: Vec<&str> = line.split(" ").collect();
            scores.insert(String::from(current_dir.to_owned() + " " + file[1]),file[0]);
            // let file2: Vec<&str> = current_dir.split(" ").collect();
            // if file2.len() >= 2{
            //     let new = file2[0].to_owned() + " " + file2[1];
            //     let value = file[0].parse::<i32>().unwrap();
            //     let old_value = scores2.get(&new).unwrap_or(&0);
            //     let new_value = old_value + value;
            //     scores2.insert(new, new_value);
            // }
            // if current_dir == "/"{
            //     println!("hello");
            //     let value = file[0].parse::<i32>().unwrap();
            //     let old_value = scores2.get("/").unwrap_or(&0);
            //     let new_value = old_value + value;
            //     scores2.insert("/".to_string(), new_value);
            // }
        }
        if line.contains("cd .."){
            let reform = current_dir.to_owned();
            let file: Vec<&str> = reform.split(" ").collect();
            current_dir = "".to_string();
            for i in 0..file.len()-1 {
                if current_dir.is_empty(){
                    current_dir = file[i].to_string();
                } else {
                    current_dir = current_dir + " " + file[i].to_string().as_str();
                }
            }
        }
    }
    
    // let map = std::collections::HashMap::from(scores2);
    // let map_value: i32 = map.values().sum();
    let (dir_vec,not_directories) = parse_directory(scores.clone());
    let (overall,overall3) = part1(dir_vec.clone(),not_directories.clone(),scores.clone());
    let needed_value = part2(dir_vec,not_directories,scores,overall3);
    println!("Day 7: Part 1: {:?}", overall);
    println!("Day 7: Part 2: {:?}", needed_value);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
fn parse_directory(scores: std::collections::HashMap<String, &str>) -> (Vec<String>,Vec<String>){
    let iter = scores.clone().into_iter();
    let mut dir_vec: Vec<String> = vec![];
    let mut not_directories: Vec<String> = vec![];
    for t in iter {
        if t.1 == "0" {
            dir_vec.push(t.0);
        } else {
            not_directories.push(t.0);
        }
    }
    return (dir_vec,not_directories)
}
fn part1(dir_vec: Vec<String>,not_directories: Vec<String>,scores: std::collections::HashMap<String, &str>) -> (i32,i32){
    let mut overall: i32 = 0;
    let mut overall3: i32 = 0;
    for dir in dir_vec{
        let mut value: i32 = 0;
        for not in not_directories.clone(){
            if not.contains(&dir){
                let t = scores.get(&not).unwrap().parse::<i32>().unwrap();
                value = value + t;
            }
        }
        if value < 100000{
            overall = overall + value
        }

        let check: Vec<&str> = dir.split(" ").collect();
        if check.len() == 2{
            overall3 = overall3 + value;
        }
        //println!("{}",overall3);
    };
    return (overall,overall3)
}
fn part2(dir_vec: Vec<String>,not_directories: Vec<String>,scores: std::collections::HashMap<String, &str>, overall3: i32) -> i32{
    let check_used_space = 70000000 - overall3;
    let check_needed_space = 30000000 - check_used_space;
    let mut needed_value: Vec<i32> = vec![];
    for dir in dir_vec{
        let mut value: i32 = 0;
        for not in not_directories.clone(){
            if not.contains(&dir){
                let t = scores.get(&not).unwrap().parse::<i32>().unwrap();
                value = value + t;
            }
        }
        if value > check_needed_space {
            needed_value.push(value);
        }
    };
    needed_value.sort();
    return needed_value[0]
}

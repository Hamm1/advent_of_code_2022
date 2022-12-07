
pub fn day7(path: String){
    let mut scores = std::collections::HashMap::new();
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
        }
        // ! This one is bad, directories from input can be multiple characters
        if line.contains("cd .."){
            let reform = current_dir.to_owned();
            let file: Vec<char> = reform.replace(" ","").chars().collect();
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
    let t = scores.get("/").unwrap();
    println!("{:?}",t);
    //scores.insert(String::from("/ a"),"1000000");
    let iter = scores.clone().into_iter();
    //let iter2 = scores.clone().into_iter();
    let mut dir_vec: Vec<String> = vec![];
    let mut not_directories: Vec<String> = vec![];
    for t in iter {
        if t.1 == "0" {
            println!("{} {} <- is a directory", t.0, t.1);
            dir_vec.push(t.0);
        } else {
            println!("{} {}", t.0, t.1);
            not_directories.push(t.0);
        }
    }
    let dir_vec2 = dir_vec.to_owned();
    let mut overall: i32 = 0;
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
        println!("{}",value);
    };
    println!("{:?}", dir_vec2);
    println!("{:?}", not_directories);
    println!("{:?}", overall);

}
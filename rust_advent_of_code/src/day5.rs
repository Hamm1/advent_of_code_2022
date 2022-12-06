
pub fn day5(path: String){
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    let mut group: Vec<String> = vec![];
    let mut group2: Vec<String> = vec![];
    for line in contents.lines(){ 
        if line.contains("["){
            group2.push(line.to_string())
        } else if line.contains("move"){
            group.push(line.to_string());
        }
    };
    let mut main_crate_test: Vec<Vec<String>> = vec![];
    let mut main_crate_convert: Vec<Vec<char>> = vec![];
    let mut i = (group2[1].len())/4+1;
    while i>0{
        main_crate_test.push(vec![]);
        i = i - 1;
    }
    for g in group2{
        let group3 = g.split(" ").collect::<Vec<&str>>();
        let mut i = 1;
        for g3 in group3{
            if !g3.is_empty(){
                let newg = g3.replace("[", "").replace("]", "");
                i = i + 4;
                main_crate_test[(i/4)-1].push(newg);
            } else {
                i = i + 1;
            }
        }
    }
    for mc in main_crate_test.clone(){
        let mut new_string: String = "".to_string();
        for m in mc{
            new_string = new_string.to_owned() + m.as_str();
        }
        new_string = new_string.chars().rev().collect();
        let new_string2: Vec<char> = new_string.chars().collect();
        main_crate_convert.push(new_string2);
        
    }
    let mut main_crate_convert2 = main_crate_convert.to_owned();
    for g in group{
        let object: Vec<&str> = g.split(" ").collect();
        let test1 = object[1].to_string().parse::<usize>().unwrap();
        let test2 = object[3].to_string().parse::<usize>().unwrap();
        let test3 = object[5].to_string().parse::<usize>().unwrap();
        // ! Part 1
        let mut i = test1;
        while i != 0{
            let length = (main_crate_convert2[test2-1]).len()-1;
            let check = main_crate_convert2[test2-1][length].to_owned();
            main_crate_convert2[test2-1].remove(length);
            main_crate_convert2[test3-1].push(check);
            i = i - 1;
        }
        // ! Part 2
        let mut i = test1;
        while i != 0{
            let length = (main_crate_convert[test2-1]).len()-i;
            let check = main_crate_convert[test2-1][length].to_owned();
            main_crate_convert[test2-1].remove(length);
            main_crate_convert[test3-1].push(check);
            i = i - 1;
        }
    }
    output(main_crate_convert2, 1);
    output(main_crate_convert, 2);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
fn output(main_crate: Vec<Vec<char>>, part:i32){
    let mut i = main_crate.len();
    let mut output: String = "".to_string();
    while i>0{
        output = output.to_owned() + &(main_crate[i-1][(main_crate[i-1]).len()-1]).to_string();
        i = i - 1;
    }
    println!("Day 5 Part {part}: Total {:?}",output.chars().rev().collect::<String>());
}
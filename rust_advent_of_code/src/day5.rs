
pub fn day5(path: String){
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    let mut group: Vec<String> = vec![];
    for line in contents.lines(){ 
        group.push(line.to_string())
    };
    let contents = std::fs::read_to_string("/home/matt/advent_of_code_2022/files/day5crate.txt").unwrap();
    let mut group2: Vec<String> = vec![];
    for line in contents.lines(){ 
        group2.push(line.to_string())
    };
    println!("{:?}",(group2[1].len())/4+1);
    
    //let group3 = group2[1].split(" ").collect::<Vec<&str>>();
    let mut main_crate_test: Vec<Vec<&str>> = vec![];
    let mut i = (group2[1].len())/4+1;
    while i>0{
        main_crate_test.push(vec![]);
        i = i - 1;
    }
    for g in group2{
        //println!("{:?}",(check_g.split(" ").collect::<Vec<&str>>()));
        let group3 = g.split(" ").collect::<Vec<&str>>();
        let mut i = 1;
        for g3 in group3{
            if !g3.is_empty(){
                let newg = g3.replace("[", "").replace("]", "");
                i = i + 4;
                main_crate_test[(i/4)-1].push(&newg);
                println!("{}",(i/4))
            } else {
                i = i + 1;
            }
        }
    }
    println!("{:?}",main_crate_test);
    // let crate_1: Vec<&str> = vec!["Z","N"];
    // let crate_2: Vec<&str> = vec!["M","C","D"];
    // let crate_3: Vec<&str> = vec!["P"];
    let crate_1: Vec<&str> = vec!["B","Z","T"];
    let crate_2: Vec<&str> = vec!["V","H","T","D","N"];
    let crate_3: Vec<&str> = vec!["B","F","M","D"];
    let crate_4: Vec<&str> = vec!["T","J","G","W","V","Q","L"];
    let crate_5: Vec<&str> = vec!["W","D","G","P","V","F","Q","M"];
    let crate_6: Vec<&str> = vec!["V","Z","Q","G","H","F","S"];
    let crate_7: Vec<&str> = vec!["Z","S","N","R","L","T","C","W"];
    let crate_8: Vec<&str> = vec!["Z","H","W","D","J","N","R","M"];
    let crate_9: Vec<&str> = vec!["M","Q","L","F","D","S"];
    // let mut main_crate: Vec<Vec<&str>> = vec![crate_1.clone(),crate_2.clone(),crate_3.clone()];
    // let mut main_crate1: Vec<Vec<&str>> = vec![crate_1.clone(),crate_2.clone(),crate_3.clone()];
    let mut main_crate: Vec<Vec<&str>> = vec![crate_1.clone(),crate_2.clone(),crate_3.clone(),crate_4.clone(),crate_5.clone(),crate_6.clone(),crate_7.clone(),crate_8.clone(),crate_9.clone()];
    let mut main_crate1: Vec<Vec<&str>> = vec![crate_1.clone(),crate_2.clone(),crate_3.clone(),crate_4.clone(),crate_5.clone(),crate_6.clone(),crate_7.clone(),crate_8.clone(),crate_9.clone()];
    println!("{:?}",main_crate);
    for g in group{
        let object: Vec<&str> = g.split(" ").collect();
        let test1 = object[1].to_string().parse::<usize>().unwrap();
        let test2 = object[3].to_string().parse::<usize>().unwrap();
        let test3 = object[5].to_string().parse::<usize>().unwrap();
        //println!("{:?}",object);
        // ! Part 1
        let mut i = test1;
        while i != 0{
            let length = (main_crate[test2-1]).len()-1;
            let check = main_crate[test2-1][length];
            main_crate[test2-1].remove(length);
            main_crate[test3-1].push(check);
            i = i - 1;
        }
        // ! Part 2
        let mut i = test1;
        while i != 0{
            let length = (main_crate_test[test2-1]).len()-i;
            let check = main_crate_test[test2-1][length];
            main_crate_test[test3-1].push(check);
            main_crate_test[test2-1].remove(length);
            i = i - 1;
        }
    }
    output(main_crate);
    output(main_crate1);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
fn output(main_crate: Vec<Vec<&str>>){
    let mut i = main_crate.len();
    let mut output: String = "".to_string();
    while i>0{
        output = output.to_owned() + (main_crate[i-1][(main_crate[i-1]).len()-1]);
        i = i - 1;
    }
    println!("Total {:?}",output.chars().rev().collect::<String>());
}

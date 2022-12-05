
pub fn day4(path: String){
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    let mut group: Vec<&str> = vec![];
    let mut total: i32 = 0;
    let mut total2: i32 = 0;
    for line in contents.lines(){ 
        group.push(line)
    }
    for g in group{
        let object: Vec<&str> = g.split(",").collect();
        let mut check;
        let mut group2: Vec<Vec<i32>> = vec![];
        for o in object{
            let object2: Vec<i32> = o.split("-").map(|x|->i32{x.parse().unwrap()}).collect();
            group2.push(object2);
        }
        // ! Part 1
        (total,check) = parse_group(group2.clone(),total,0,0,1,0,0,1,1,1);
        if !check{
            (total,_) = parse_group(group2.clone(),total,1,0,0,0,1,1,0,1);
        }
        // ! Part 2
        (total2,check) = parse_group(group2.clone(),total2,0,0,1,1,0,1,1,0);
        if !check{
            (total2,_) = parse_group(group2,total2,1,1,0,0,1,0,0,1);
        }
    }
    println!("Day 4 Part 1: Full Overlap {}",total);
    println!("Day 4 Part 2: Partial Overlap {}",total2);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
fn parse_group(group2: Vec<Vec<i32>>,mut total2:i32,p1:usize,p2:usize,p3:usize,p4:usize,p5:usize,p6:usize,p7:usize,p8:usize) -> (i32,bool){
    if (group2[p1][p2]) <= (group2[p3][p4]) {
        if (group2[p5][p6]) >= (group2[p7][p8]){
            total2 = total2 + 1;
            return (total2,true);
        }
    }
    return (total2,false);
}
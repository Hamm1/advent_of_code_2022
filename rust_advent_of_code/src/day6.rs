
pub fn day6(path: String){
    let run_other_parts = false;
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    // ! way 1
    parse(contents.clone(),4, 1);
    parse(contents.clone(),14, 2);
    if run_other_parts{
        // ! way 2
        parse2(contents.clone(), 4);
        parse2(contents.clone(), 14);
        // ! way 3
        parse3(contents.clone(), 4);
        parse3(contents, 14);
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
fn parse(contents: String, length: usize, part: usize){
    let group: Vec<char> = contents.chars().collect();
    let group2: Vec<char> = contents.chars().collect();
    let mut i = 0;
    let mut m = 0;
    'outer: for _g in group{
        let mut new_string: String = "".to_string();
        let mut check = new_string.len();
        while check < length {
            new_string = new_string.to_owned() + &group2[i].to_string();
            check = new_string.len();
            i = i + 1;
        }
        let new_vec: Vec<char> = new_string.chars().collect();
        let new_vec2: Vec<char> = new_string.chars().collect();
        let mut x = 0;
        let mut w = length;
        'outer2: for new in new_vec{
            for c in 1..w{
                if new == new_vec2[x+c]{
                    break 'outer2
                }
            }
            w = w - 1;
            x = x + 1;
            if x == length-1{
                //println!("{:?}",new_vec2);
                println!("Day 6 Part {}: Total {:?}",part,i);
                break 'outer
            }
        }
        m = m + 1;
        i = m;
    }
}
fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}
fn parse2(contents: String, length: usize){
    let group: Vec<char> = contents.chars().collect();
    let group2: Vec<char> = contents.chars().collect();
    let mut i = 0;
    let mut m = 0;
    for _g in group{
        let mut new_string: String = "".to_string();
        let mut check = new_string.len();
        while check < length {
            new_string = new_string.to_owned() + &group2[i].to_string();
            check = new_string.len();
            i = i + 1;
        }
        m = m + 1;
        i = m;
        //println!("{:?}",new_string);
        match unique(new_string.as_str()){
            None => break,
            Some((_i,_j,_c)) => continue
        };
    }
    println!("{:?}",i+length-1);
}
fn parse3(contents: String, length: usize){
    let group: Vec<char> = contents.chars().collect();
    let group2: Vec<char> = contents.chars().collect();
    let mut i = 0;
    let mut m = 0;
    for _g in group{
        let mut new_string: String = "".to_string();
        let mut check = new_string.len();
        while check < length {
            new_string = new_string.to_owned() + &group2[i].to_string();
            check = new_string.len();
            i = i + 1;
        }
        //println!("{:?}",new_string);
        m = m + 1;
        i = m;
        let mut x = 0;
        let mut w = 0;
        let new_vec: Vec<char> = new_string.chars().collect();
        let new_vec2: Vec<char> = new_string.chars().collect();
        for new in new_vec{
            for n in 0..length-1{
                if new == new_vec2[n]{
                    w = w + 1;
                }
                x = x + 1;
            }
        }
        if w == length-1{
            println!("{}",i+length-1);
            break
        }
    }
}
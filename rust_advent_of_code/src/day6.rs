
pub fn day6(path: String){
    let run_other_parts = false;
    let start = std::time::Instant::now();
    let contents = std::fs::read_to_string(path).unwrap();
    let group: Vec<char> = contents.chars().collect();
    //let contents = include_bytes!("/home/matt/advent_of_code_2022/files/day6.txt");
    // ! way 5 Wnner for speed and understanding
    parse5(group.clone(),4, 1);
    parse5(group,14, 1);
    if run_other_parts{
        // ! way 1
        parse(contents.clone(),4, 1);
        parse(contents.clone(),14, 2);
        // ! way 2
        parse2(contents.clone(), 4);
        parse2(contents.clone(), 14);
        // ! way 3
        parse3(contents.clone(), 4);
        parse3(contents.clone(), 14);
        // ! way 4 Wnner for speed
        parse4(contents.clone().as_bytes(),4, 1);
        parse4(contents.as_bytes(),14, 2);
        // ! way 6 OpenAI for the string search with modifications
        parse6(contents.clone(), 4);
        parse6(contents.clone(), 14);
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
fn parse4(contents: &[u8], length: usize, part: usize) {
    let mut position = 0;

    'outer: loop {
        for i in (1..length).rev() {
            let ch = &contents[position + i];
            for j in (0..i).rev() {
                if ch == &contents[position + j] {
                    position += j + 1;
                    continue 'outer;
                }
            }
        }
        println!("Day 6 Part {}: Total {:?}",part,position + length);
        break 'outer
    }
}
fn parse5(contents: Vec<char>, length: usize, part: usize) {
    let mut position = 0;

    'outer: loop {
        for i in (1..length).rev() {
            let ch = &contents[position + i];
            for j in (0..i).rev() {
                if ch == &contents[position + j] {
                    position += j + 1;
                    continue 'outer;
                }
            }
        }
        println!("Day 6 Part {}: Total {:?}",part,position + length);
        break 'outer
    }
}
fn parse6(contents: String, length: usize){
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
        if is_unique(&new_string){
            break
        }
    }
    println!("{:?}",i+length-1);
}
fn is_unique(s: &str) -> bool {
    let mut seen_chars: Vec<char> = vec![];
    for c in s.chars() {
        if seen_chars.contains(&c) {
            return false;
        }
        seen_chars.push(c);
    }
    return true;
}
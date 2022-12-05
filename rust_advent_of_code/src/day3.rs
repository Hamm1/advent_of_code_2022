
pub fn day3(path: String){
    let contents = std::fs::read_to_string(path).unwrap();
    let mut priority = ('a'..='z').into_iter().collect::<Vec<char>>();
    priority.extend(('A'..='Z').into_iter().collect::<Vec<char>>());
    total(priority.clone(),part1(contents.clone()),1);
    total(priority.clone(),part2(contents),2);
}
fn total(priority: Vec<char>,final_numbers: Vec<char>,part: i32){
    let mut total = 0;
    for f in final_numbers{
        let t = match priority.iter().position(|&x| x == f.clone()).ok_or("err"){
            Ok(d) => d + 1,
            Err(_) => 0
        };
        total = total + t;
    }
    println!("Day 3 Part {part}: Total {}",total);
}
fn part1(contents: String) -> Vec<char>{
    let mut final_numbers: Vec<char> = vec![];
    for line in contents.lines(){
        let split_location = line.len() / 2;
        let char_vec: Vec<char> = line.chars().collect();
        let mut first_half: Vec<char> = vec![];
        let mut second_half: Vec<char> = vec![];
        for c in &char_vec[0..split_location]{
            first_half.push(*c);
        }
        for c in &char_vec[split_location..char_vec.len()]{
            second_half.push(*c)
        }
        'outer: for f in first_half.clone(){
            for s in second_half.clone(){
                if f == s {
                    final_numbers.push(s);
                    break 'outer
                }
            }
        }
    }
    return final_numbers
}
fn part2(contents: String) -> Vec<char>{
    let mut groups_of_3_all: Vec<&str> = vec![];
    let mut final_numbers2: Vec<char> = vec![];
    for line in contents.lines(){
        groups_of_3_all.push(line)
    }
    for chunk in groups_of_3_all.chunks(3){
        if chunk.len() == 3{
            let char_vec: Vec<char> = chunk[0].chars().collect();
            let char_vec2: Vec<char> = chunk[1].chars().collect();
            let char_vec3: Vec<char> = chunk[2].chars().collect();
            'outer: for f in char_vec.clone(){
                for s in char_vec2.clone(){
                    if f == s {
                        for p in char_vec3.clone(){
                            if s == p{
                                final_numbers2.push(p);
                                break 'outer
                            }
                        }
                    }
                } 
            }
        }
    }
    return final_numbers2
}
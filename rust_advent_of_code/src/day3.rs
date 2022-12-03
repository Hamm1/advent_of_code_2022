
pub fn day3(){
    let contents = std::fs::read_to_string("/home/matt/advent_of_code_2022/files/day3.txt").unwrap();
    let mut priority = vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    let priority2 = vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
    priority.extend(priority2);
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
    total(priority.clone(),final_numbers,1);

    let mut groups_of_3_all: Vec<&str> = vec![];
    let mut final_numbers2: Vec<char> = vec![];
    for line in contents.lines(){
        groups_of_3_all.push(line)
    }
    for chunk in groups_of_3_all.chunks(3){
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
    total(priority.clone(),final_numbers2,2);
}
fn total(priority: Vec<&str>,final_numbers: Vec<char>,part: i32){
    let mut total = 0;
    for f in final_numbers{
        let t = match priority.iter().position(|&x| x == f.to_string().clone()).ok_or("err"){
            Ok(d) => d + 1,
            Err(_) => 0
        };
        total = total + t;
    }
    println!("Day 3 Part {part}: Total {}",total);
}
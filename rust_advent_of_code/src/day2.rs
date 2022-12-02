
pub fn day2(){
    let mut scores = std::collections::HashMap::new();
    scores.insert(String::from("A X"), 3+1);
    scores.insert(String::from("A Y"), 6+2);
    scores.insert(String::from("A Z"), 0+3);
    scores.insert(String::from("B X"), 0+1);
    scores.insert(String::from("B Y"), 3+2);
    scores.insert(String::from("B Z"), 6+3);
    scores.insert(String::from("C X"), 6+1);
    scores.insert(String::from("C Y"), 0+2);
    scores.insert(String::from("C Z"), 3+3);

    let mut scores2 = std::collections::HashMap::new();
    scores2.insert(String::from("A X"), 3+0);
    scores2.insert(String::from("A Y"), 1+3);
    scores2.insert(String::from("A Z"), 2+6);
    scores2.insert(String::from("B X"), 1+0);
    scores2.insert(String::from("B Y"), 2+3);
    scores2.insert(String::from("B Z"), 3+6);
    scores2.insert(String::from("C X"), 2+0);
    scores2.insert(String::from("C Y"), 3+3);
    scores2.insert(String::from("C Z"), 1+6);

    let contents = std::fs::read_to_string("/home/matt/advent_of_code_2022/files/day2.txt").unwrap();
    rps(scores, contents.clone(), "1");
    rps(scores2, contents, "2");
}

fn rps(scores: std::collections::HashMap<String, i32>, contents: String, part: &str){
    let mut total: Vec<i32> = vec![];
    for line in contents.lines(){
        let t = scores.get(line).unwrap();
        total.push(*t);
    }
    println!("Day 2 part {part}: Total Rock, Paper, Scissors score {}",total.into_iter().sum::<i32>())
}


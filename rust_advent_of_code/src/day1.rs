
pub fn day1(){
    let contents = std::fs::read_to_string("/home/matt/OneDrive/Advent_of_code/files/day1.txt").unwrap();
    let mut elf: Vec<i32> = vec![];
    let mut dump = 0;
    for line in contents.lines(){
        if line == ""{
            elf.push(dump);
            dump = 0;
        } else {
            dump = dump + line.parse::<i32>().unwrap()
        }
    }
    let most = match elf.iter().max().ok_or("err"){
        Ok(d) => d,
        Err(_) => &0
    };
    let greedest_elf = match elf.iter().position(|&x| x == most.clone()).ok_or("err"){
        Ok(d) => d + 1,
        Err(_) => 0
    };
    println!("Elf number {greedest_elf} with {most} calories");
}
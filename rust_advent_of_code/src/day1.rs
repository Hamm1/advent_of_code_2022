
pub fn day1(){
    let contents = std::fs::read_to_string("/home/matt/advent_of_code_2022/files/day1.txt").unwrap();
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
    let mut elf2 = elf.clone();
    elf2.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut large1:i32 = elf[0];
    let mut large2:i32 = 0;
    let mut large3:i32 = 0;
    let mut i:usize = 1;

    while i<elf.len()
    {
        if large1 < elf[i] {
            large2 = large1;
            large1 = elf[i];
        } else if large2 < elf[i] {
            large3 = large2;
            large2 = elf[i];
        } else if large3 < elf[i] {
            large3 = elf[i]
        }
        i = i + 1;
    }
    let most = match elf.iter().max().ok_or("err"){
        Ok(d) => d,
        Err(_) => &0
    };
    let greedest_elf = match elf.iter().position(|&x| x == most.clone()).ok_or("err"){
        Ok(d) => d + 1,
        Err(_) => 0
    };
    let greedest_elf2 = match elf.iter().position(|&x| x == large2.clone()).ok_or("err"){
        Ok(d) => d + 1,
        Err(_) => 0
    };
    let greedest_elf3 = match elf.iter().position(|&x| x == large3.clone()).ok_or("err"){
        Ok(d) => d + 1,
        Err(_) => 0
    };
    let total = elf[greedest_elf-1] + elf[greedest_elf2-1] + elf[greedest_elf3-1];
    println!("Day 1 Part 1: Elf number {greedest_elf} with {most} calories");
    println!("Day 2 Part 2: Top 3 snack elves {}, {}, {} for a total of {}",greedest_elf,greedest_elf2,greedest_elf3,total);
}
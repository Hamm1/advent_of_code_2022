
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
fn main() {
    let path = get_current_working_dir("/advent_of_code_2022");
    day1::day1(path.to_owned() + "advent_of_code_2022/files/day1.txt");
    day2::day2(path.to_owned() + "advent_of_code_2022/files/day2.txt");
    day3::day3(path.to_owned() + "advent_of_code_2022/files/day3.txt");
    day4::day4(path.to_owned() + "advent_of_code_2022/files/day4.txt");
	day5::day5(path.to_owned() + "advent_of_code_2022/files/day5.txt");
	day6::day6(path.to_owned() + "advent_of_code_2022/files/day6.txt");
}

pub fn get_current_working_dir(pattern: &str) -> String {
    let res = std::env::current_dir();
    let stuff = match res {
        Ok(path) => path.into_os_string().into_string().unwrap().to_lowercase(),
        Err(_) => "FAILED".to_string()
    };
	let mut dir = "".to_owned();
	let stuff = stuff.replace("\\", "/");
	let modified_pattern = pattern.replace("/", "");
	if stuff.contains(pattern){
		let reform: Vec<&str> = stuff.split("/").collect();
		let mut i = 0;
		for r in reform.clone(){
			if r.eq(modified_pattern.as_str().clone()){
				break;
			} else {
				i = i + 1
			}
		}
		for n in 0..i{
			dir = dir + reform[n] + "/";
		};
	}
	return dir;
}

const get = await Deno.readTextFile('../files/day7.txt')
const contents = get.split("\n").map(function(str) {
    return str; 
});
const scores: Map<string,number> = new Map();
let current_dir = "";
for (let i = 0; i < get.length; i++){
    if (contents[i]?.includes("dir")){
        const something = current_dir + " " + contents[i]?.replace("dir ", "");
        scores.set(something, 0);
    }
    if (contents[i]?.includes("$ cd") && !contents[i]?.includes("$ cd ..")){
        if (current_dir == ""){
            current_dir = contents[i].replace("$ cd ","");
        } else {
            current_dir = current_dir + " " + contents[i]?.replace("$ cd ", "");
        }
        scores.set(current_dir, 0)
    }
    if (!contents[i]?.includes("dir") && !contents[i]?.includes("$")){
        const file = contents[i]?.split(" ");
        if (file?.length == 2){
            scores.set(current_dir + " " + file[1],parseInt(file[0]))
        }
    }
    if (contents[i]?.includes("cd ..")){
        const file = current_dir.split(" ");
        current_dir = "";
        for (let i = 0; i < file.length-1; i++){
            if (current_dir == ""){
                current_dir = file[i];
            } else if (file[i] == ""){
                console.log("");
            } else {
                current_dir = current_dir + " " + file[i].trim();
            }
        }
    }
}
const [dir_array,not_directories] = parse_directory(scores);
const [overall,overall3] = part1(dir_array,not_directories,scores);
const _needed_value = part2(dir_array, not_directories,scores,overall3);
console.log(overall);
console.log(overall3);

function parse_directory(scores: Map<string,number>):[string[],string[]] {
    const dir_array: string[] = [];
    const not_directories: string[] = [];
    for(const [key, value] of scores.entries()){
        if (value === 0){
            dir_array.push(key);
        } else {
            not_directories.push(key);
        }
    }
    return [dir_array,not_directories]
}
function part1(dir_array: string[],not_directories:string[],scores: Map<string,number>): [number,number]{
    let overall = 0;
    let overall3 = 0;
    for(let i = 0; i < dir_array.length; i++){
        let value = 0;
        for(let m = 0; m < not_directories.length; m++){
            if (not_directories[m]?.includes(dir_array[i])){
                const t: number = scores.get(not_directories[m])!;
                value = value + t;
            }
        }
        if (value < 100000){
            overall = overall + value;
        }
        const check = dir_array[i].split(" ");
        if (check.length === 2){
            overall3 = overall3 + value;
        }
    }
    return [overall,overall3]
}
function part2(dir_array: string[],not_directories: string[],scores: Map<string,number>,overall3: number): number{
    const check_used_space = 70000000 - overall3;
    const check_needed_space = 30000000 - check_used_space;
    const needed_value: number[] = [];
    for (const dir of dir_array){
        let value = 0;
        for (const not of not_directories){
            if (not?.includes(dir)){
                const t: number = scores.get(not)!;
                value = value + t;
            }
        }
        if (value > check_needed_space) {
            needed_value.push(value);
        }
    }
    const check = needed_value.sort();
    return check[0];
}
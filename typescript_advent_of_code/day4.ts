
const start = new Date().getTime();
const get = await Deno.readTextFile('../files/day4.txt')
const contents = get.split("\n").map(function(str) {
    return str; 
});

let total = 0;
let total2 = 0;
for(let i = 0; i < contents.length; i++){
    const group2: number[][] = [];
    const object: string[] = contents[i].split(",");
    let check = false;
    for(let i = 0; i < object.length; i++){
        const test = object[i].split("-").map(x=>parseInt(x));
        group2.push(test);
    }
    // ! Part 1
    [total,check] = parse_group(group2,total,0,0,1,0,0,1,1,1);
    if (!check){
        [total,check] = parse_group(group2,total,1,0,0,0,1,1,0,1);
    }
    // ! Part 2
    [total2,check] = parse_group(group2,total2,0,0,1,1,0,1,1,0);
    if (!check){
        [total2,check] = parse_group(group2,total2,1,1,0,0,1,0,0,1);
    }
}

console.log(`Day 4 Part 1: Full Overlap ${total}`)
console.log(`Day 4 Part 2: Partial Overlap ${total2}`)
const elapsed = new Date().getTime() - start;
console.log(`Time to run ${elapsed/1000}`)

function parse_group(group2: number[][], total: number,p1: number,p2: number,p3: number,p4: number,p5: number,p6: number,p7: number,p8: number):[number,boolean]{
    if (group2[p1][p2] <= group2[p3][p4]) {
        if (group2[p5][p6] >= group2[p7][p8]){
            total++;
            return [total,true];
        }
    }
    return [total,false];
}
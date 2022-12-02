
const get = await Deno.readTextFile('/home/matt/advent_of_code_2022/files/day1.txt')
const new_get = get.split("\n").map(function(str) {
    return parseInt(str.replace("","0")); 
});
const elf: Array<number> = []
let dump = 0
for(let i = 0; i < new_get.length; i++){
    if(new_get[i] === 0){
        elf.push(dump)
        dump = 0
    } else {
        dump = dump + new_get[i]
    }
}
const most = Math.max(...elf)
const greedest_elf = elf.indexOf(most) + 1
console.log(`Elf number ${greedest_elf} with ${most} calories`)

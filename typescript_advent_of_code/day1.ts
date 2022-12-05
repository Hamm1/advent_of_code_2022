
const get = await Deno.readTextFile('../files/day1.txt')
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
let index = elf.indexOf(most);
elf.splice(index,1);
const most2 = Math.max(...elf)
let index2 = elf.indexOf(most2);
elf.splice(index2,1);
const most3 = Math.max(...elf);
let index3 = elf.indexOf(most3);
console.log(`Elf number ${index+1} with ${most} calories`)
console.log(`Day 2 Part 2: Top 3 snack elves ${index+1}, ${index2+2}, ${index3+1} for a total of ${most + most2 +most3}`)

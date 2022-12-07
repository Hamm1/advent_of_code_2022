
function isUnique(str: string): boolean {
    const seenChars = new Set<string>();
    for (const char of str) {
      if (seenChars.has(char)) {
        return false;
      }
      seenChars.add(char);
    }
    return true;
}

async function main(length: number){
    const get = await Deno.readTextFile('../files/day6.txt')
    for(let i = 0; i < get.length; i++){
        let new_string: string = "";
        for(let m = i; m < length+i; m++){
            new_string += get.charAt(m);
        }
        if (isUnique(new_string)){
            console.log(i+length)
            break
        }
    }
}
main(4);
main(14);
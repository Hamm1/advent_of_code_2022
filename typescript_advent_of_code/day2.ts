day2();
async function day2(){
    let scores: Map<string,number> = new Map();
    scores.set("A X", 3+1);
    scores.set("A Y", 6+2);
    scores.set("A Z", 0+3);
    scores.set("B X", 0+1);
    scores.set("B Y", 3+2);
    scores.set("B Z", 6+3);
    scores.set("C X", 6+1);
    scores.set("C Y", 0+2);
    scores.set("C Z", 3+3);

    let scores2: Map<string,number> = new Map();
    scores2.set("A X", 3+0);
    scores2.set("A Y", 1+3);
    scores2.set("A Z", 2+6);
    scores2.set("B X", 1+0);
    scores2.set("B Y", 2+3);
    scores2.set("B Z", 3+6);
    scores2.set("C X", 2+0);
    scores2.set("C Y", 3+3);
    scores2.set("C Z", 1+6);

    const get = await Deno.readTextFile('../files/day2.txt')
    const contents = get.split("\n").map(function(str) {
        return str; 
    });
    rps(scores,contents,1);
    rps(scores2,contents,2);
}

function rps(scores: Map<string,number>, contents: string[], part: number){
    let total: number = 0;
    for(let i = 0; i < contents.length; i++){
        for(let [key, value] of scores.entries()){
            if (key === contents[i]){
                total = total + value;
            }
        } 
    }
    console.log(`Day 2 part ${part}: Total Rock, Paper, Scissors score ${total}`);
}

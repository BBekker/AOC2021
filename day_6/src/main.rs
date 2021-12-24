
use std::io;
use std::io::BufRead;

const DAYS : usize= 256;
const RESET_VALUE : usize= 6;
const INITIAL_VALUE : usize = 8;

fn main() {
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().unwrap().split(",").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut counts : Vec<i64> = vec![0;INITIAL_VALUE+1];

    input.iter().for_each(|x| counts[*x as usize] += 1);
    //dbg!(&counts);

    for _ in 0..DAYS {
        
        let mut newcounts : Vec<i64> = vec![0;INITIAL_VALUE+1];
        newcounts[INITIAL_VALUE] = counts[0];
        newcounts[RESET_VALUE] = counts[0];
        for i in 0..INITIAL_VALUE {
            newcounts[i] += counts[i+1];
        }
        counts = newcounts;
        //dbg!{&counts};
    }
    println!("{} lanternfish", counts.iter().sum::<i64>());
}


// 120 fish: 787 ms
// With size hint: 610 ms
// Add to vec instead of copies : 192 ms
// smarter script: 20 ms
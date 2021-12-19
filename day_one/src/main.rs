
use std::io;
use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut history = VecDeque::<f64>::with_capacity(3);
    let mut previous = 1000000000000.0;
    let mut count = 0;

    history.push_back(lines.next().unwrap().unwrap().parse::<f64>().unwrap());
    history.push_back(lines.next().unwrap().unwrap().parse::<f64>().unwrap());

    for line in lines {
        history.push_back(line.unwrap().parse::<f64>().unwrap());
        let newval = history.iter().fold(0.0, |acc, &x| acc + x) / 3.0;
        print!("{}", newval);

        if newval > previous
        {
            count += 1;
            print!(" count");
        }
        previous = newval;
        history.pop_front();
        print!("\r\n");
    }

    print!("{}", count);
}

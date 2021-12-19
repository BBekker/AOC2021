
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut depth : i64 = 0;
    let mut horizontal : i64 = 0;
    let mut aim : i64 = 0;
    for line in lines {
        let unwrap = line.unwrap();
        let mut split = unwrap.split(' ');
        let cmd = split.next().unwrap();
        let number : i64 = split.next().unwrap().parse::<i64>().unwrap();

        match cmd 
        {
            "up" => {
                aim-= number;
            }
            "down" => {
                aim+= number;
            }
            "forward" => {
                horizontal += number; 
                depth+= aim * number;
            }
            &_ => {}
        }
        //println!("{} {}", depth, horizontal);
    }

    println!("{} {}", depth, horizontal);
    println!("{}", depth * horizontal);
}

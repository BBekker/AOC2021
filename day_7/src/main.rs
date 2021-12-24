
use std::io;
use std::io::BufRead;


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().unwrap().split(",").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut best_fuel_used = 9999999999999;
    let mut best_pos = 0;
    for i in 0.. *input.iter().max().unwrap() {

        let fuel_used : i64= input.iter().map(|x| (x - i).abs() * ((x-i).abs() + 1 ) / 2  ).sum();
        if fuel_used < best_fuel_used {
            best_fuel_used = fuel_used;
            best_pos = i;
        }
    }

    println!("{} {}", best_fuel_used, best_pos);
}

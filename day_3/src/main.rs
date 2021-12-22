
use std::io;
use std::io::BufRead;


const BITS: usize = 12;

fn most_common(numbers: &Vec::<i32> ) -> [bool; BITS]
{
    let mut counts : [i32; BITS] = [0;BITS];
    for number in numbers {
        for x in 0..BITS {
            counts[x] += (number >> x) & 1;
        }
    }
    dbg!(counts);
    let mut ret : [bool; BITS] = [false;BITS];
    for x in 0..BITS {
        ret[x] = counts[x] * 2 >= (numbers.len() as i32); //This line screwed me up by using integer division.
    }
    return ret;
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut input = Vec::<i32>::new();

    for line in lines {
        let number = i32::from_str_radix(&line.unwrap(),2).unwrap();
        input.push(number)
    }
    input.sort();
    input.dedup();

    print_numbers(&input);

    let mut oxigen_candidates = input.clone();
    for bit in (0..BITS).rev() {
        dbg!(bit);
        let common_bits = most_common(&oxigen_candidates);
        dbg!(common_bits);
        oxigen_candidates.retain(|x| (((x >> bit)&1) != 0) == common_bits[bit] );
        print_numbers(&oxigen_candidates);
        if oxigen_candidates.len() == 1 {
            break;
        }
    }

    let mut co2_candidates = input.clone();
    for bit in (0..BITS).rev() {
   //     dbg!(bit);
        let common_bits = most_common(&co2_candidates);
   //     dbg!(common_bits);
        co2_candidates.retain(|x| (((x >> bit)&1) != 0) != common_bits[bit] );
   //     dbg!(&co2_candidates);
        if co2_candidates.len() == 1 {
            break;
        }
    }


    assert_eq!(co2_candidates.len(), 1);
    assert_eq!(oxigen_candidates.len(), 1);

    println!("{} {}", oxigen_candidates[0], co2_candidates[0]);
    println!("{}", oxigen_candidates[0] * co2_candidates[0]);
}

fn print_bits(number : i32)
{
    for bit in (0..BITS).rev()
    {
        print!("{}",number >> bit & 1);
    }
}

fn print_numbers(numbers: &Vec::<i32>)
{
    println!{"["};
    for number in numbers{
        print_bits(*number);
        println!{","};
    }
    println!{"]"};
}
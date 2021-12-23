//Trying to push rust functional programming
// Its not great

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let draws = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();

    let boards_text = lines.map(|x| x.unwrap()).collect::<Vec<_>>(); //Sadly we have to do this to do the chunks afterwards.
    for board_text in boards_text.chunks_exact(6) {
        let board: Vec<Vec<i32>> = board_text[1..]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        boards.push(board);
    }

    let mut has_won = vec![false; boards.len()];
    for i in 0..draws.len() {
        let had_won = has_won.clone();
        for (nboard, board) in boards.iter().enumerate().filter(|(ix, _)| !had_won[*ix]) {
            let board_state = board
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|item| draws[0..i + 1].contains(item))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let full_row = board_state
                .iter()
                .map(|row| !row.contains(&false))
                .any(|x| x);
            let full_column = (0..board_state[0].len())
                .map(|i| board_state.iter().map(|row| row[i]).all(|x| x))
                .any(|x| x);

            if full_row || full_column {
                //board has won
                let mut score = 0;
                for (ix, row) in board.iter().enumerate() {
                    for (jx, value) in row.iter().enumerate() {
                        if !board_state[ix][jx] {
                            score += value;
                            print!("{} ", value)
                        } else {
                            print!("[{}] ", value)
                        }
                    }
                    println!();
                }
                println! {"board {} won: draw {}, score: {}, result: {}", nboard, draws[i], score, score*draws[i]};
                has_won[nboard] = true;
            }
        }
    }
}

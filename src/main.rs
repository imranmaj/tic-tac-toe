use std::{
    io::{self, stdin, BufRead},
    string::ToString,
};

mod board;
mod player;
use board::{Board, GameResult};

fn print_board(board: &Board) {
    println!();
    println!("{}", board.to_string());
}

fn main() -> io::Result<()> {
    let mut board = Board::new();
    while let GameResult::Incomplete = board.check_game_end() {
        print_board(&board);
        let turn = board.current_turn().clone();
        println!("{:?}'s turn", turn);

        let mut buf = String::new();
        stdin().lock().read_line(&mut buf)?;
        let mut locs = buf
            .split(',')
            .map(|loc| loc.trim().parse::<usize>().expect("expected number 1 to 3") - 1);
        let pos = (
            locs.next().expect("expected at least 2 positions"),
            locs.next().expect("expected at least 2 positions"),
        );
        if let Err(msg) = board.add_move(turn, pos) {
            println!("{}", msg);
            continue;
        }

        board.next_move();
    }

    print_board(&board);
    if let GameResult::Win(winner) = board.check_game_end() {
        println!("{:?} wins", winner);
    } else {
        println!("It's a tie");
    }
    println!("press enter to continue...");
    let _ = stdin().lock().read_line(&mut String::new());
    Ok(())
}

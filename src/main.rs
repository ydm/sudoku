use std::io::{self, Read};
use sudoku::presets;
use sudoku::rules::{ExclusionRule, SingleOptRule};
use sudoku::structure::{Solver, Sudoku};

#[allow(dead_code)]
fn read() -> Sudoku {
    let mut buffer = String::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_to_string(&mut buffer).unwrap();
    buffer.parse().unwrap()
}

fn main() {
    // let mut s = read();
    let mut s = presets::load_hard();
    println!("{}\n", s);

    s.rules.push(Box::new(ExclusionRule::new_row()));
    s.rules.push(Box::new(ExclusionRule::new_col()));
    s.rules.push(Box::new(ExclusionRule::new_square()));
    s.rules.push(Box::new(SingleOptRule::new_row()));
    s.rules.push(Box::new(SingleOptRule::new_col()));
    s.rules.push(Box::new(SingleOptRule::new_square()));
    println!("Solved: {}", s.solve());
    println!("{}", s);
}

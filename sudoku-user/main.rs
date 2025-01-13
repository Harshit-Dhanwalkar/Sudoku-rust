use std::io::{self, Write};
use std::time::Instant;

fn solve_sudoku(board: &mut Vec<Vec<u8>>) -> bool {
    fn is_valid(board: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;

        for i in 0..9 {
            if board[row][i] == num || board[i][col] == num || board[box_row + i / 3][box_col + i % 3] == num {
                return false;
            }
        }
        true
    }

    fn backtrack(board: &mut Vec<Vec<u8>>, row: usize, col: usize) -> bool {
        if row == 9 {
            return true;
        }

        let (next_row, next_col) = if col == 8 { (row + 1, 0) } else { (row, col + 1) };

        if board[row][col] != 0 {
            return backtrack(board, next_row, next_col);
        }

        for num in 1..=9 {
            if is_valid(board, row, col, num) {
                board[row][col] = num;

                if backtrack(board, next_row, next_col) {
                    return true;
                }

                board[row][col] = 0;
            }
        }

        false
    }

    backtrack(board, 0, 0)
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn input_board() -> Vec<Vec<u8>> {
    let mut board = vec![vec![0; 9]; 9];
    println!("Enter your Sudoku board row by row (use 0 for empty cells):");

    for i in 0..9 {
        loop {
            print!("Row {}: ", i + 1);
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let row: Vec<u8> = input
                .trim()
                .split_whitespace()
                .filter_map(|x| x.parse::<u8>().ok())
                .collect();

            if row.len() == 9 {
                board[i] = row;
                break;
            } else {
                println!("Invalid input. Please enter exactly 9 numbers (0-9) separated by spaces.");
            }
        }
    }

    board
}

fn main() {
    let mut board = input_board();

    println!("\nOriginal Sudoku:");
    print_board(&board);

    let start_time = Instant::now();

    if solve_sudoku(&mut board) {
        let duration = start_time.elapsed();
        println!("\nSolved Sudoku:");
        print_board(&board);
        println!("\nTime taken to solve: {:?}", duration);
    } else {
        let duration = start_time.elapsed();
        println!("\nThe Sudoku is not solvable!");
        println!("\nTime taken to determine unsolvability: {:?}", duration);
    }
}

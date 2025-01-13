use std::time::Instant;

fn solve_sudoku_optimized(board: &mut Vec<Vec<u8>>) -> bool {
    let mut rows = vec![0u16; 9];
    let mut cols = vec![0u16; 9];
    let mut boxes = vec![0u16; 9];

    // Initialize bitmasks
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] != 0 {
                let num = 1 << (board[row][col] - 1);
                rows[row] |= num;
                cols[col] |= num;
                boxes[(row / 3) * 3 + col / 3] |= num;
            }
        }
    }

    fn backtrack(
        board: &mut Vec<Vec<u8>>,
        rows: &mut Vec<u16>,
        cols: &mut Vec<u16>,
        boxes: &mut Vec<u16>,
        row: usize,
        col: usize,
    ) -> bool {
        if row == 9 {
            return true;
        }

        let (next_row, next_col) = if col == 8 { (row + 1, 0) } else { (row, col + 1) };

        if board[row][col] != 0 {
            return backtrack(board, rows, cols, boxes, next_row, next_col);
        }

        let box_idx = (row / 3) * 3 + col / 3;
        for num in 1..=9 {
            let bit = 1 << (num - 1);
            if rows[row] & bit == 0 && cols[col] & bit == 0 && boxes[box_idx] & bit == 0 {
                board[row][col] = num as u8;
                rows[row] |= bit;
                cols[col] |= bit;
                boxes[box_idx] |= bit;

                if backtrack(board, rows, cols, boxes, next_row, next_col) {
                    return true;
                }

                board[row][col] = 0;
                rows[row] &= !bit;
                cols[col] &= !bit;
                boxes[box_idx] &= !bit;
            }
        }

        false
    }

    backtrack(board, &mut rows, &mut cols, &mut boxes, 0, 0)
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let mut board = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    println!("Original Sudoku:");
    print_board(&board);

    // Start timing
    let start_time = Instant::now();

    if solve_sudoku_optimized(&mut board) {
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

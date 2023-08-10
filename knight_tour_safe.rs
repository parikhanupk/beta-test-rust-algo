use std::time::Instant;
use std::cell::Cell;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;



fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    let num_calls: Box<Cell<u64>> = Box::new(Cell::new(0));

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &offsets, 0, 0, 1, &num_calls);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
    println!("Calls: {:?}", num_calls.get());

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}



// Display the board.
fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            print!("{:<02} ", board[r][c]);
        }
        println!();
    }
    println!();
}



// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &[[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
    num_calls: &Box<Cell<u64>>
) -> bool {
    num_calls.set(num_calls.get() + 1);
    if num_visited == INUM_ROWS * INUM_COLS {
        if REQUIRE_CLOSED_TOUR == false {
            return true;
        } else {
            for m in offsets.iter() {
                let row = cur_row + m[0];
                let col = cur_col + m[1];
                if row >= 0
                    && row < INUM_ROWS
                    && col >= 0
                    && col < INUM_COLS
                    && board[row as usize][col as usize] == 0
                {
                    return true;
                }
            }
            return false;
        }
    } else {
        for m in offsets.iter() {
            let row = cur_row + m[0];
            let col = cur_col + m[1];

            //skip where target is off board or already visited
            if row < 0
                || row >= INUM_ROWS
                || col < 0
                || col >= INUM_COLS
                || board[row as usize][col as usize] != UNVISITED
            {
                continue;
            }

            //valid row and col giving unvisited target
            board[row as usize][col as usize] = num_visited;
            if find_tour(board, &offsets, row, col, num_visited + 1, num_calls) == true {
                return true;
            }

            //backtrack - undo the move as this state can't find solution
            board[row as usize][col as usize] = UNVISITED;
        }
        return false;
    }
}

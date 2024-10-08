use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::process::exit;

const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const MAKE_MR_BAOCHUN_HAPPY: bool = true;

fn print_board(board: &[[char; 8]; 8]) {
    if MAKE_MR_BAOCHUN_HAPPY {
        println!("  abcdefgh");
        for i in 0..8 {
            print!("{} ", (('a' as u8) + (i as u8)) as char);
            for j in 0..8 {
                print!("{}", board[i][j]);
            }
            println!();
        }
    } else {
        println!("  a b c d e f g h");
        for i in 0..8 {
            print!("{} ", (('a' as u8) + (i as u8)) as char);
            for j in 0..8 {
                print!("{} ", board[i][j]);
            }
            println!();
        }
    }
}

fn set_up_board() -> [[char; 8]; 8] {
    let mut board = [['.'; 8]; 8];
    board[3][3] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';
    board
}

fn get_moves(board: &[[char; 8]; 8], colour: char) -> HashSet<(usize, usize)> {
    let mut moves = HashSet::new();
    let anti_colour = if colour == 'W' { 'B' } else { 'W' };
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] != '.' {
                continue;
            }
            for (dx, dy) in DIRS.iter() {
                let mut nx = i as isize + dx;
                let mut ny = j as isize + dy;
                if (nx < 0 || nx >= 8 || ny < 0 || ny >= 8)
                    || board[nx as usize][ny as usize] != anti_colour
                {
                    continue;
                }
                while nx >= 0
                    && nx < 8
                    && ny >= 0
                    && ny < 8
                    && board[nx as usize][ny as usize] == anti_colour
                {
                    nx += dx;
                    ny += dy;
                }
                if nx >= 0
                    && nx < 8
                    && ny >= 0
                    && ny < 8
                    && board[nx as usize][ny as usize] == colour
                {
                    moves.insert((i, j));
                    break;
                }
            }
        }
    }
    moves
}

fn parse_input(input: &String) -> (usize, usize) {
    let input = input.trim();
    if input.len() != 2 {
        return (99, 99);
    }

    let x = input.chars().nth(0).expect("Failed to get the first character.");
    let y = input.chars().nth(1).expect("Failed to get the second character.");
    let x = ((x as u8) - ('a' as u8)) as usize;
    let y = ((y as u8) - ('a' as u8)) as usize;

    if x >= 8 || y >= 8 {
        return (99, 99);
    }

    (x, y)
}

fn run_game(board: &mut [[char; 8]; 8]) -> i32 {
    let mut black_turn = false;
    loop {
        // get possible moves
        let white_moves = get_moves(&board, 'W');
        let black_moves = get_moves(&board, 'B');
        
        // check if game is over and determine next turn
        if white_moves.len() == 0 && black_moves.len() == 0 {
            break; // has winner
        } else { // does not have winner, prompt for moves
            print_board(&board);
            if black_turn {
                if white_moves.len() == 0 {
                    println!("W player has no valid move.");
                    black_turn = true;
                } else {
                    black_turn = false;
                }
            } else {
                print_board(&board);
                if black_moves.len() == 0 {
                    println!("B player has no valid move.");
                    black_turn = false;
                } else {
                    black_turn = true;
                }
            }
        }
        let colour = if black_turn { 'B' } else { 'W' };
        let anti_colour = if colour == 'W' { 'B' } else { 'W' };

        // get user input
        let (x, y) = loop {
            // print prompt
            print!("Enter move for colour {} (RowCol): ", colour);
            io::stdout().flush().expect("Failed to flush stdout.");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input");

            // parse input
            let (x, y) = parse_input(&input);

            // validate input
            if (black_turn && black_moves.contains(&(x, y)))
                || (!black_turn && white_moves.contains(&(x, y)))
            {
                break (x, y);
            } else {
                println!("Invalid move. Try again.");
                if MAKE_MR_BAOCHUN_HAPPY { print_board(&board); }
            }
        };

        // update board
        board[x][y] = colour;
        for (dx, dy) in DIRS.iter() {
            let mut nx = x as isize + dx;
            let mut ny = y as isize + dy;
            while nx >= 0
                && nx < 8
                && ny >= 0
                && ny < 8
                && board[nx as usize][ny as usize] == anti_colour
            {
                nx += dx;
                ny += dy;
            }
            if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 && board[nx as usize][ny as usize] == colour {
                nx -= dx;
                ny -= dy;
                while nx != x as isize || ny != y as isize {
                    board[nx as usize][ny as usize] = colour;
                    nx -= dx;
                    ny -= dy;
                }
            }
        }
    }

    if MAKE_MR_BAOCHUN_HAPPY {
        print_board(&board);
        if black_turn {
            println!("W player has no valid move.");
            println!("B player has no valid move.");
        } else {
            println!("B player has no valid move.");
            println!("W player has no valid move.");
        }
    }

    // get scores
    let mut white_score = 0;
    let mut black_score = 0;
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == 'B' {
                black_score += 1;
            } else if board[i][j] == 'W' {
                white_score += 1;
            }
        }
    }

    // print winner
    if white_score > black_score {
        println!("White wins by {} points!", white_score - black_score);
    } else if black_score > white_score {
        println!("Black wins by {} points!", black_score - white_score);
    } else {
        println!("Draw!");
    }

    0
}

fn main() {
    let mut board = set_up_board();
    exit(run_game(&mut board));
}

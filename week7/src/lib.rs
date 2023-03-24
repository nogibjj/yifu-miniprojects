/// "Given a number, return a string."
///
/// The function takes a single parameter, move_num, which is a u32. The function returns a &'static str
///
/// Arguments:
///
/// * `move_num`: u32 - This is the move number that we're converting to a string.
///
/// Returns:
///
/// A string slice
pub fn move_to_string(move_num: u32) -> &'static str {
    match move_num {
        1 => "rock",
        2 => "paper",
        3 => "scissors",
        _ => unreachable!(),
    }
}

// Determine the winner of a game of rock, paper, scissors
pub fn determine_winner(player_move: u32, computer_move: u32) -> i8 {
    if player_move == computer_move {
        return 0;
    }
    match player_move {
        1 => {
            if computer_move == 2 {
                -1
            } else {
                1
            }
        }
        2 => {
            if computer_move == 3 {
                -1
            } else {
                1
            }
        }
        3 => {
            if computer_move == 1 {
                -1
            } else {
                1
            }
        }
        _ => unreachable!(),
    }
}

use std::io;

use rand::Rng;
use week7::determine_winner;
use week7::move_to_string;
fn main() {
    // Print a welcome message
    println!("Welcome to Rock-Paper-Scissors!");

    loop {
        // Prompt the user to choose a move or exit the game
        println!("Please choose your move:");
        println!("1. Rock");
        println!("2. Paper");
        println!("3. Scissors");
        println!("4. Exit game");

        // Read the user's input from the console
        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        // Parse the user's input into a number or exit the game if 4 is entered
        let player_move: u32 = match player_move.trim() {
            "4" => return,
            input => match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please choose a valid move.");
                    continue; // If the input cannot be parsed, prompt the user again
                }
            },
        };

        // Check that the player's move is valid
        if !(1..=3).contains(&player_move) {
            println!("Invalid move. Please choose a valid move.");
            continue; // If the move is invalid, prompt the user again
        }

        // Generate a random move for the computer
        let computer_move = rand::thread_rng().gen_range(1..=3);

        // Print the player's move and the computer's move
        println!("You chose {}.", move_to_string(player_move));
        println!("Computer chose {}.", move_to_string(computer_move));

        // Determine the winner of the round
        let result = determine_winner(player_move, computer_move);

        // Print the result of the round
        match result {
            0 => println!("It's a tie!"),
            1 => println!("You win!"),
            -1 => println!("Computer wins!"),
            _ => panic!("Unexpected result: {}", result),
        }
    }
}

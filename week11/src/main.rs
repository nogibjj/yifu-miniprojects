use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

struct Question {
    prompt: String,
    options: Vec<String>,
    answer: usize,
    points: u32,
}

fn main() {
    let questions = [
        Question {
            prompt: String::from("What is the capital of France?"),
            options: vec![
                String::from("London"),
                String::from("Paris"),
                String::from("Berlin"),
            ],
            answer: 1,
            points: 10,
        },
        Question {
            prompt: String::from("What is the highest mountain in the world?"),
            options: vec![
                String::from("K2"),
                String::from("Mount Everest"),
                String::from("Makalu"),
            ],
            answer: 1,
            points: 10,
        },
        Question {
            prompt: String::from("Who invented the telephone?"),
            options: vec![
                String::from("Alexander Graham Bell"),
                String::from("Thomas Edison"),
                String::from("Nikola Tesla"),
            ],
            answer: 0,
            points: 10,
        },
    ];

    let mut score = 0;
    let mut time_remaining = Duration::from_secs(60);
    let start_time = Instant::now();
    let mut leaderboard: HashMap<String, u32> = HashMap::new();

    println!("Welcome to the Quiz Game!");

    for question in &questions {
        println!("{}", question.prompt);

        // Shuffle the options randomly
        let mut shuffled_options = question.options.clone();
        shuffled_options.shuffle(&mut rand::thread_rng());

        for (i, option) in shuffled_options.iter().enumerate() {
            println!("{}: {}", i + 1, option);
        }

        let mut user_answer = String::new();
        io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");
        let user_answer = user_answer.trim().parse::<usize>().expect("Invalid answer");

        if shuffled_options[user_answer - 1] == question.options[question.answer] {
            println!("Correct! You earn {} points.", question.points);
            score += question.points;
        } else {
            println!(
                "Sorry, the correct answer is: {}",
                question.options[question.answer]
            );
        }

        time_remaining = time_remaining
            .checked_sub(start_time.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if time_remaining == Duration::from_secs(0) {
            println!("Time's up! Your final score is: {}", score);
            break;
        }
    }

    println!("Your final score is: {}", score);

    if score > 0 {
        println!("Enter your name to save your score:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let name = name.trim().to_string();

        if let Some(existing_score) = leaderboard.get_mut(&name) {
            if score > *existing_score {
                println!(
                    "Congratulations, you beat your previous high score of {} points!",
                    existing_score
                );
                *existing_score = score;
            } else {
                println!(
                    "Your score of {} points is not higher than your previous high score of {} points.",
                    score, existing_score
                );
            }
        } else {
            leaderboard.insert(name, score);
            println!(
                "Congratulations, you have a new high score of {} points!",
                score
            );
        }
    }

    println!("Leaderboard:");
    for (name, score) in &leaderboard {
        println!("{}: {}", name, score);
    }
}

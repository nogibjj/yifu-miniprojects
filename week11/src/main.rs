use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::{Duration, Instant};

struct Question {
    prompt: String,
    options: Vec<String>,
    answer: usize,
    points: u32,
}

fn main() -> io::Result<()> {
    let mut questions: Vec<Question> = vec![];
    let file = File::open("questions.txt")?;
    let reader = BufReader::new(file);

    // Read questions from the file
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        let prompt = parts[0].to_string();
        let options = parts[1..=4].iter().map(|s| s.to_string()).collect();
        let answer = parts[5].parse::<usize>().expect("Invalid answer");
        let points = parts[6].parse::<u32>().expect("Invalid points");
        let question = Question {
            prompt,
            options,
            answer,
            points,
        };
        questions.push(question);
    }

    // Shuffle the questions randomly
    let mut rng = rand::thread_rng();
    questions.shuffle(&mut rng);

    let mut score = 0;
    let mut time_remaining = Duration::from_secs(60);
    let start_time = Instant::now();
    let mut leaderboard: HashMap<String, u32> = HashMap::new();

    println!("Welcome to the Quiz Game!");

    for question in &questions[0..5] {
        println!("{}", question.prompt);

        // Shuffle the options randomly
        let mut shuffled_options = question.options.clone();
        shuffled_options.shuffle(&mut rng);

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
                    score,
                    existing_score
                );
            }
        } else {
            leaderboard.insert(name, score);
        }
    }

    println!("Leaderboard:");
    for (name, score) in &leaderboard {
        println!("{}: {}", name, score);
    }

    Ok(())
}

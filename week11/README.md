# Rust Quiz Game

This Rust code is a simple quiz game that asks three questions and allows the user to answer them. The questions are stored in an array of `Question` structs. Each `Question` struct has a `prompt`, an array of `options`, an `answer`, and a number of `points`.

## Getting Started

To get started with this project, you'll need to have Rust installed on your machine. You can download Rust from the official website [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and run the following command to start the game:

`cargo run`


## How to Play

The game starts by initializing variables for the score, time remaining, start time, and leaderboard. It then prints a welcome message and loops through each question. For each question, it shuffles the options randomly and prints them out. It then reads in the user's answer and compares it to the correct answer. If the user's answer is correct, it adds the number of points for that question to the score. After all questions have been answered, it prints out the final score.
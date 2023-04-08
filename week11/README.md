# Quiz Game

This is a simple quiz game written in Rust. The game reads questions from a file and presents them to the user. The user has 60 seconds to answer as many questions as possible. The game keeps track of the user's score and displays a leaderboard at the end.

## Usage

To run the game, simply execute the following command:

`cargo run`

## Questions

The questions are stored in a file called `questions.txt`. Each line of the file represents a single question and has the following format:

prompt,option1,option2,option3,option4,answer,points


- `prompt`: The text of the question.
- `option1`-`option4`: The possible answers to the question.
- `answer`: The index of the correct answer (1-4).
- `points`: The number of points awarded for a correct answer.


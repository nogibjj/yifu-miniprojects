//A command-line tool to Get the square of a number
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Yifu", about = "")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yifu", about = "")]
    Squarenum {
        #[clap(short, long)]
        number: String,
    },
}

//this is the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Squarenum { number }) => {
            let num: i32 = number.parse().unwrap();
            let result = num * num;
            println!("The square of {} is {}", number, result);
        }
        None => {
            println!("No command was used");
        }
    }
}
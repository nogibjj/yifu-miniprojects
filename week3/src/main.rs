//A command-line tool to convert a decimal to hexadecimal
use clap::Parser;
use decimaltohexadecimal::decimaltohexadecimal;

#[derive(Parser)]
#[clap(version = "1.0", author = "Yifu", about = "convert a decimal to hexadecimal")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yifu", about = "convert a decimal to hexadecimal")]
    Decimaltohexadecimal {
        #[clap(short, long)]
        number: String,
    },
}

//this is the main function
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Decimaltohexadecimal { number }) => {
            let number = number.parse::<i32>().unwrap();
            println!("{}", decimaltohexadecimal(number));
        }
        None => println!("No command was used"),
    }
}

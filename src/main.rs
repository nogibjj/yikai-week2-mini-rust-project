//A command-line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yikai Liu",
    about = "Check if two files are different"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yikai Liu")]
    Check {
        #[clap(short, long)]
        file1: String,
        file2: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Check { file1, file2 }) => {
            let result = hello::check_diff(&file1, &file2);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}

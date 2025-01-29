/*
A Command-line tool to analyze lyrics to songs and put them into a sqlite database.
 */
use clap::Parser;
use sqlite_huggingface::{classify_lyrics, get_all_zeroshotcandidates, read_lyrics};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Rashid Rasul",
    about = "A Command-line tool to analyse lyrics to songs and put them into a sqlite database."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Rashid Rasul")]
    Classify {
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
    Candidates {},
    Lyrics {
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Classify { file }) => {
            println!("Classify {}", file);
            let lyrics = read_lyrics(&file);
            // Print lyrics to console
            for line in read_lyrics(&file) {
                println!("{}", line);
            }
            // Use classify_lyrics() from lib.rs to classify lyrics
            match classify_lyrics(lyrics) {
                Ok(predictions) => {
                    for prediction in predictions {
                        for label in prediction {
                            println!("{}: {}", label.text, label.score);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error classifying lyrics: {}", e);
                }
            }
        }
        // use get_all_zeroshotcandidates() from lib.rs to get all candidates
        Some(Commands::Candidates {}) => {
            for candidate in get_all_zeroshotcandidates() {
                println!("{}", candidate);
            }
        }
        Some(Commands::Lyrics { file }) => {
            println!("Lyrics {}", file);
            for line in read_lyrics(&file) {
                println!("{}", line);
            }
        }
        None => {
            println!("No command given");
        }
    }
}

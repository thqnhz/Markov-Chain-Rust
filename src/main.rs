use std::{collections::HashMap};

use clap::{Parser, Subcommand};
use rand::{rngs::ThreadRng, seq::{IndexedRandom, IteratorRandom}};

#[derive(Parser, Debug)]
#[command(author = "ThanhZ", version = "0.3.0", about = "Some Markov Chain Implementations")]
struct Cli {
    #[command(subcommand)]
    game: Games,
}

#[derive(Subcommand, Debug)]
enum Games {
    /// Gambler's ruin simulation
    Gambler {
        /// Starting money
        #[arg(short, long)]
        money: u16,
        /// Bid per game
        #[arg(short, long)]
        bid: u16,
    },
    /// Sentence Generator
    Sentence {
        /// String input
        #[arg(short, long)]
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.game {
        Games::Gambler { money, bid } =>
            match gamblers_ruin(&mut (*money as i32), &(*bid as i32)) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e)
            },
        Games::Sentence { input } => println!("{}", sentence_generator(input))
    };
    
}

fn gamblers_ruin(money: &mut i32, bid: &i32) -> Result<String, String> {
    if bid > money || *money / bid > 1000 {
        Err("Bid is too big or too small".to_string())
    } else {
        let target_money: i32 = *money * 2;
        let mut attempt: u16 = 1;
        while *money > 0 && *money < target_money {
            *money += if rand::random_bool(0.5) {1} else {-1} * bid;
            attempt += 1;
        }
        let result = String::from(format!("After {} attempts, you ", attempt.to_string()));
        if *money <= 0 {
            Ok(result + "lost it all at the casino! RIPBOZO")
        } else {
            Ok(result + "doubled your money! $$$")
        }
    }
}

fn sentence_generator(input: &String) -> String {
    let words: Vec<&str> = input.trim().split(" ").collect();
    let mut next_words: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..words.len() {
        let current_word: String = words[i].to_lowercase();
        let next_word: String = if i + 1 < words.len() {
            words[i+1].to_lowercase()
        } else {
            "".to_string()
        };
        next_words.entry(current_word)
            .or_insert_with(|| vec!["".to_string()])
            .push(next_word);
    }
    next_words.insert("".to_string(), vec!["".to_string()]);

    // RNG part
    let mut rng: ThreadRng = rand::rng();
    let starting_words: Vec<&String> = next_words.keys().collect();

    let mut current_word: &String = *starting_words
        .clone()
        .iter()
        .filter(|&&k| k != "")
        .choose(&mut rng)
        .expect("Not enough info from input");

    let mut result: String = String::from("A wise man once said: ");

    loop {
        if current_word == "" {
            break;
        }
        result.push_str(current_word);
        result.push_str(" ");
        if let Some(next_vec) = next_words.get(current_word) {
            let next_word_option = next_vec.choose(&mut rng);
            match next_word_option {
                Some(next_word) => current_word = next_word,
                None => break,
            }
        } else { break; }
    }
    return result;
}

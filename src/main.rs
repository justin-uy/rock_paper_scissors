extern crate rand;
extern crate regex;

use std::io;
use rand::Rng;
use regex::Regex;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl ToString for Outcome {
    fn to_string(&self) -> String {
        match *self {
            Outcome::Win => String::from("Win"),
            Outcome::Lose => String::from("Lose"),
            Outcome::Draw => String::from("Draw"),
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl ToString for Choice {
    fn to_string(&self) -> String {
        match *self {
            Choice::Rock => String::from("rock"),
            Choice::Paper => String::from("paper"),
            Choice::Scissors => String::from("scissors"),
        }
    }
}

impl Choice {
    fn rand() -> Choice {
        match rand::thread_rng().gen_range(0, 3) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => panic!("impossible value"),
        }
    }

    fn parse(s: String) -> Result<Self, String> {
        let trimmed = s.trim();
        let re = Regex::new(r"(?i)(rock|paper|scissors)").unwrap();
        let parsed = match re.captures(trimmed) {
            None => String::from(""),
            Some(captures) => String::from(&captures[0]).to_lowercase(),
        };

        if parsed.len() != 0 && parsed.len() != trimmed.len() {
            println!("{} must be german for {}", trimmed, parsed);
        }

        match parsed.as_ref() {
            "rock" => Ok(Choice::Rock),
            "paper" => Ok(Choice::Paper),
            "scissors" => Ok(Choice::Scissors),
            _ => Err(format!("Invalid choice: {}!", trimmed)),
        }
    }

    fn outcome_against(&self, c2: &Self) -> Outcome {
        if self.eq(c2) {
            return Outcome::Draw;
        } else if self.eq(&Choice::Rock) && c2.eq(&Choice::Paper) {
            return Outcome::Lose;
        } else if self.eq(&Choice::Paper) && c2.eq(&Choice::Scissors) {
            return Outcome::Lose;
        } else if self.eq(&Choice::Scissors) && c2.eq(&Choice::Rock) {
            return Outcome::Lose;
        }

        Outcome::Win
    }
}

struct Player {
    choice: Option<Choice>,
    is_human: bool,
}

impl Player {
    fn new(is_human: bool) -> Self {
        let choice: Option<Choice>;
        if !is_human {
            choice = Some(Choice::rand());
        } else {
            choice = None;
        }

        Player {
            choice: choice,
            is_human: is_human,
        }
    }

    fn get_choice(&mut self) {
        if !self.is_human {
            return;
        }

        println!("Make a choice:");
        let mut choice_string = String::new();

        io::stdin().read_line(&mut choice_string).expect(
            "Failed to readline",
        );

        match Choice::parse(choice_string) {
            Ok(c) => {
                self.choice = Some(c);
            }
            Err(e) => {
                println!("{}", e);
                self.get_choice()
            }
        }
    }

    fn play(&self, opponent: &Self) -> Result<Outcome, String> {
        if self.choice.is_none() || opponent.choice.is_none() {
            return Err("Both players must have made a choice".to_string());
        }

        Ok(self.choice.unwrap().outcome_against(
            &opponent.choice.unwrap(),
        ))
    }
}

fn main() {
    println!("Let's play rock, paper, scissors");
    let mut player = Player::new(true);
    let computer = Player::new(false);

    player.get_choice();

    println!("Your choice: {}", player.choice.unwrap().to_string());
    println!(
        "Computer's choice: {}",
        computer.choice.unwrap().to_string()
    );

    println!("{}!", player.play(&computer).unwrap().to_string());
}

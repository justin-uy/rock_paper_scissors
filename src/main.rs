extern crate rand;
extern crate regex;

use std::io;
use rand::Rng;
use regex::Regex;

#[derive(Debug, PartialEq)]
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


#[derive(Debug, Copy, Clone, PartialEq)]
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

    fn parse(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        let re = Regex::new(r"(?i)(rock|paper|scissors)").expect("this must be a valid regex");
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

    fn request_choice(&mut self) {
        if !self.is_human {
            return;
        }

        println!("Make a choice:");
        let mut choice_string = String::new();

        io::stdin().read_line(&mut choice_string).expect("Failed to readline");

        match Choice::parse(choice_string.as_ref()) {
            Ok(c) => {
                self.choice = Some(c);
            }
            Err(e) => {
                println!("{}", e);
                self.request_choice()
            }
        }
    }

    fn play(&self, opponent: &Self) -> Result<Outcome, String> {
        match (self.choice, opponent.choice) {
            (Some(c1), Some(c2)) => Ok(c1.outcome_against(&c2)),
            _ => Err("Both players must have made a choice".to_string()),

        }
    }
}

fn main() {
    println!("Let's play rock, paper, scissors");
    let mut player = Player::new(true);
    let computer = Player::new(false);

    player.request_choice();

    println!("Your choice: {}", player.choice.unwrap().to_string());
    println!("Computer's choice: {}",
             computer.choice.unwrap().to_string());

    println!("{}!", player.play(&computer).unwrap().to_string());
}


#[cfg(test)]
mod tests {
    use Player;
    use Outcome;
    use Choice;


    #[test]
    fn test_outcome_to_string() {
        assert_eq!(Outcome::Win.to_string(), "Win");
        assert_eq!(Outcome::Lose.to_string(), "Lose");
        assert_eq!(Outcome::Draw.to_string(), "Draw");
    }

    #[test]
    fn test_choice_to_string() {
        assert_eq!(Choice::Rock.to_string(), "rock");
        assert_eq!(Choice::Paper.to_string(), "paper");
        assert_eq!(Choice::Scissors.to_string(), "scissors");
    }

    #[test]
    fn test_choice_parse() {

        // normal cases
        assert_eq!(Choice::parse(String::from("rock")).unwrap(), Choice::Rock);
        assert_eq!(Choice::parse(String::from("paper")).unwrap(), Choice::Paper);
        assert_eq!(Choice::parse(String::from("scissors")).unwrap(),
                   Choice::Scissors);

        // cases with excess characters
        assert_eq!(Choice::parse(String::from("RockKK")).unwrap(), Choice::Rock);
        assert_eq!(Choice::parse(String::from("paperzzzz")).unwrap(),
                   Choice::Paper);
        assert_eq!(Choice::parse(String::from("12341234scissorsacdkakd")).unwrap(),
                   Choice::Scissors);

        // should use first match
        assert_eq!(Choice::parse(String::from("RockPaperScissors")).unwrap(),
                   Choice::Rock);
    }

    #[test]
    fn test_non_human_characters_should_have_some_choice() {
        let human = Player::new(true);
        assert_eq!(human.choice, None);

        let non_human = Player::new(false);
        assert_eq!(non_human.choice.is_some(), true);
    }

    #[test]
    fn test_players_must_have_choices_to_play() {
        let mut p1 = Player::new(true);
        let mut p2 = Player::new(true);
        assert_eq!(p1.choice, None);
        assert_eq!(p2.choice, None);
        assert_eq!(p1.play(&p2).is_err(), true);

        p1.choice = Some(Choice::Rock);
        assert_eq!(p1.play(&p2).is_err(), true);

        p2.choice = Some(Choice::Rock);
        assert_eq!(p1.play(&p2).is_ok(), true);
    }

    #[test]
    fn test_outcome_against() {
        assert_eq!(Choice::Rock.outcome_against(&Choice::Rock), Outcome::Draw);
        assert_eq!(Choice::Rock.outcome_against(&Choice::Paper), Outcome::Lose);
        assert_eq!(Choice::Rock.outcome_against(&Choice::Scissors),
                   Outcome::Win);
        assert_eq!(Choice::Paper.outcome_against(&Choice::Scissors),
                   Outcome::Lose);
        assert_eq!(Choice::Paper.outcome_against(&Choice::Rock), Outcome::Win);
        assert_eq!(Choice::Scissors.outcome_against(&Choice::Rock),
                   Outcome::Lose);
        assert_eq!(Choice::Scissors.outcome_against(&Choice::Paper),
                   Outcome::Win);
    }
}

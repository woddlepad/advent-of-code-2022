use std::fs;

enum GameState {
    Loss,
    Draw,
    Win,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
enum ParseRPSError {
    Unknown(String),
}

impl RPS {
    pub const fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn vs(&self, other: &RPS) -> i32 {
        let value = self.value();
        let win = 6;
        let draw = 3;
        let loss = 0;
        let game_value = match self {
            RPS::Paper => match other {
                RPS::Paper => draw,
                RPS::Rock => win,
                RPS::Scissors => loss,
            },
            RPS::Rock => match other {
                RPS::Paper => loss,
                RPS::Rock => draw,
                RPS::Scissors => win,
            },
            RPS::Scissors => match other {
                RPS::Paper => win,
                RPS::Rock => loss,
                RPS::Scissors => draw,
            },
        };
        return value + game_value;
    }

    fn derive_throw_from_desired_outcome(&self, desired_state: GameState) -> RPS {
        return match self {
            RPS::Paper => match desired_state {
                GameState::Win => RPS::Scissors,
                GameState::Loss => RPS::Rock,
                GameState::Draw => RPS::Paper,
            },
            RPS::Rock => match desired_state {
                GameState::Win => RPS::Paper,
                GameState::Loss => RPS::Scissors,
                GameState::Draw => RPS::Rock,
            },
            RPS::Scissors => match desired_state {
                GameState::Win => RPS::Rock,
                GameState::Loss => RPS::Paper,
                GameState::Draw => RPS::Scissors,
            },
        };
    }
}

impl std::str::FromStr for RPS {
    type Err = ParseRPSError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(ParseRPSError::Unknown(s.to_string())),
        }
    }
}

impl std::str::FromStr for GameState {
    type Err = ParseRPSError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameState::Loss),
            "Y" => Ok(GameState::Draw),
            "Z" => Ok(GameState::Win),
            _ => Err(ParseRPSError::Unknown(s.to_string())),
        }
    }
}

fn parse_game(game_str: &str) -> (RPS, RPS) {
    let throws: Vec<RPS> = game_str
        .split(" ")
        .map(|throw| {
            throw
                .parse::<RPS>()
                .expect("Should have had valid throw in game.")
        })
        .collect();
    let my_throw: &RPS = throws.get(1).expect("Should have my throw");
    let other_throw: &RPS = throws.get(0).expect("Should have opponent throw");
    return (my_throw.clone(), other_throw.clone());
}

fn parse_game_alt(game_str: &str) -> (RPS, RPS) {
    let variables: Vec<&str> = game_str.split(" ").collect();

    let other_throw: RPS = variables.get(0).unwrap().parse().unwrap();
    let desired_state: GameState = variables.get(1).unwrap().parse().unwrap();
    let my_throw: RPS = other_throw.derive_throw_from_desired_outcome(desired_state);
    return (my_throw, other_throw);
}

fn main() {
    let file_path = "./strategy.txt";
    let strategy_guide =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let throws: Vec<(RPS, RPS)> = strategy_guide
        .split("\n")
        .map(|game_str| parse_game_alt(game_str))
        .collect();

    throws.iter().for_each(|(my_throw, other_throw)| {
        println!(
            "ME({:?}, {}) vs OTHER({:?}, {}) = {}",
            my_throw,
            my_throw.value(),
            other_throw,
            other_throw.value(),
            my_throw.vs(other_throw)
        );
    });

    let game_scores = throws
        .iter()
        .map(|(my_throw, other_throw)| my_throw.vs(other_throw));

    let score: i32 = game_scores.sum();
    println!("{}", score);
}

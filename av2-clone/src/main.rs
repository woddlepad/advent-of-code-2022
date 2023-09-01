#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RPSThrow {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
enum RPSState {
    Win,
    Loss,
    Draw,
}

// this is a text
fn get_rps_state(throw: &RPSThrow, other_throw: &RPSThrow) -> RPSState {
    match (throw, other_throw) {
        (throw, other_throw) if throw == other_throw => RPSState::Draw,
        (RPSThrow::Rock, RPSThrow::Scissors) => RPSState::Win,
        (RPSThrow::Paper, RPSThrow::Rock) => RPSState::Win,
        (RPSThrow::Scissors, RPSThrow::Paper) => RPSState::Win,
        _ => RPSState::Loss,
    }
}

fn get_rps_state_value(state: &RPSState) -> i8 {
    match state {
        RPSState::Win => 3,
        RPSState::Loss => 0,
        RPSState::Draw => 1,
    }
}

fn get_random_throw() -> RPSThrow {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let throw = match rng.gen_range(0..3) {
        0 => RPSThrow::Rock,
        1 => RPSThrow::Paper,
        _ => RPSThrow::Scissors,
    };
    return throw;
}

fn main() {
    let throw = RPSThrow::Paper;
    let other_throw = get_random_throw();
    let result = get_rps_state(&throw, &other_throw);
    let value = get_rps_state_value(&result);
    println!(
        "{:?} vs {:?} => {:?}({:?})",
        throw, other_throw, result, value
    );
}

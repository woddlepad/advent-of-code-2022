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

fn main() {
    let throw = RPSThrow::Paper;
    let other_throw = RPSThrow::Rock;
    let result = get_rps_state(&throw, &other_throw);
    let value = get_rps_state_value(&result);
    println!(
        "{:?} vs {:?} => {:?}({:?})",
        throw, other_throw, result, value
    );
}

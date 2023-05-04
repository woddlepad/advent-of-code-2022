use std::{collections::VecDeque, fs, str::FromStr};

struct CraneInstruction {
    from: usize,
    to: usize,
    amount: i32,
}

impl CraneInstruction {
    fn execute_one_at_a_time(&self, crate_stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.amount {
            let moving_item = crate_stacks[self.from]
                .pop()
                .expect("Instruction failed to execute");
            crate_stacks[self.to].push(moving_item);
        }
    }
    fn execute_in_chunks(&self, crate_stacks: &mut Vec<Vec<char>>) {
        let mut items_to_move: VecDeque<char> = VecDeque::new();
        for _ in 0..self.amount {
            items_to_move.push_front(
                crate_stacks[self.from]
                    .pop()
                    .expect("Instruction failed to execute"),
            );
        }
        for idx in 0..self.amount as usize {
            crate_stacks[self.to].push(items_to_move[idx])
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCraneError;

impl FromStr for CraneInstruction {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terms = s.split(" ").collect::<Vec<&str>>();
        let amount: i32 = terms[1].parse().unwrap();
        let from: usize = terms[3].parse::<usize>().unwrap() - 1;
        let to: usize = terms[5].parse::<usize>().unwrap() - 1;

        return Ok(CraneInstruction { from, to, amount });
    }

    type Err = ParseCraneError;
}

fn main() {
    let file_path = "./crates.txt";
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split_content: Vec<&str> = file_content.split("\n\n").collect();
    let (crate_layout, instruction_description) = (split_content[0], split_content[1]);
    let crate_layout_lines: Vec<&str> = crate_layout.lines().collect();

    let num_crates = (crate_layout_lines[0].len() + 2) / 4;
    let mut crate_stacks: Vec<Vec<char>> =
        Vec::from_iter(std::iter::repeat(Vec::new()).take(num_crates));

    crate_layout_lines.iter().for_each(|line| {
        let line_chars = line.chars().collect::<Vec<char>>();
        for crate_index in 0..num_crates {
            let crate_char = line_chars[crate_index * 4 + 1];

            if crate_char.is_ascii_uppercase() {
                crate_stacks[crate_index].insert(0, crate_char);
            }
        }
    });

    // println!("{}", num_crates);
    // println!("{:?}", crate_stacks);

    let instructions = instruction_description
        .lines()
        .map(|line| line.parse::<CraneInstruction>().unwrap());

    for instruction in instructions {
        instruction.execute_in_chunks(&mut crate_stacks);
    }

    let top_stack = crate_stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&'\0').to_string())
        .collect::<Vec<_>>()
        .join("");
    // println!("{:?}", crate_stacks);
    println!("{}", top_stack);
}

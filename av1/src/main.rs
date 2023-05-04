use std::fs;

fn count_calories(calorie_list_as_str: String) -> Vec<i32> {
    let parse_err_msg = "Expected perfect input";
    return calorie_list_as_str
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| y.parse::<i32>().expect(parse_err_msg))
                .sum()
        })
        .collect();
}

fn main() {
    let file_path = "./calories.txt";
    let calorie_list_as_str =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut counted_calories = count_calories(calorie_list_as_str.to_string());

    let max_index = counted_calories
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .expect("Expected max value");

    counted_calories.sort_unstable_by(|a, b| b.cmp(a));

    println!(
        "The {}. elf is carrying {} calories and is therefore carrying the most.",
        max_index + 1,
        counted_calories[0]
    );

    let top_three: i32 = counted_calories[..=2].iter().sum();
    println!("The top 3 elfs are carrying {} calories", top_three);
}

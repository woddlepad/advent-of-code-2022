use std::collections::HashSet;
use std::fs;

fn split_into_compartments(rucksack: &str) -> (&str, &str) {
    let len = rucksack.len();
    assert!(len % 2 == 0);
    let middle = len / 2;
    return (&rucksack[..middle], &rucksack[middle..]);
}

fn find_duplicate(a: &str, b: &str) -> Option<char> {
    let items_a: HashSet<char> = a.chars().collect::<HashSet<char>>();
    let items_b: HashSet<char> = b.chars().collect::<HashSet<char>>();
    let intersection = items_a.intersection(&items_b);

    return intersection.map(|item| item.clone()).next();
}

fn find_duplicate_three(a: &str, b: &str, c: &str) -> Option<char> {
    let items_a: HashSet<char> = a.chars().collect::<HashSet<char>>();
    let items_b: HashSet<char> = b.chars().collect::<HashSet<char>>();
    let items_c: HashSet<char> = c.chars().collect::<HashSet<char>>();

    let intersection_a_b = items_a
        .intersection(&items_b)
        .map(|x| x.clone())
        .collect::<HashSet<char>>();
    let intersection_a_b_c = intersection_a_b.intersection(&items_c);

    return intersection_a_b_c.map(|item| item.clone()).next();
}

fn get_item_value(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        return c as u8 - 96;
    } else if c.is_ascii_uppercase() {
        return c as u8 - 38;
    } else {
        panic!("Invalid item");
    }
}

fn main() {
    let file_path = "./rucksack.txt";
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rucksack_description: Vec<&str> = file_content.lines().collect();

    let rucksack_value_sum: u32 = rucksack_description
        .iter()
        .map(|rucksack| split_into_compartments(&rucksack))
        .map(|(com_a, com_b)| find_duplicate(com_a, com_b).unwrap())
        .map(|c| get_item_value(c) as u32)
        .sum();

    let elf_badge_value_sum: u32 = rucksack_description
        .chunks(3)
        .map(|x| find_duplicate_three(x[0], x[1], x[2]).unwrap())
        .map(|c| get_item_value(c) as u32)
        .sum();

    println!("{:?} rucksack value sum", rucksack_value_sum);
    println!("{:?} elf badge sum", elf_badge_value_sum);
}

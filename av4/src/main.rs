use std::{fs, ops::Range};

trait RangeContainsRange<Range> {
    fn fully_contains(&self, other: &Range) -> bool;
    fn overlaps(&self, other: &Range) -> bool;
}

impl RangeContainsRange<Range<i32>> for Range<i32> {
    fn fully_contains(&self, other: &Range<i32>) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
    fn overlaps(&self, other: &Range<i32>) -> bool {
        return (self.start <= other.start && self.end >= other.start)
            || (self.end >= other.end && self.start <= other.end)
            || self.fully_contains(other)
            || other.fully_contains(self);
    }
}

fn main() {
    let file_path = "./ranges.txt";
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let ranges = file_content
        .lines()
        .map(|x| {
            let ranges_vec = x
                .split(",")
                .map(|str_range| {
                    let range_vec: Vec<_> = str_range
                        .split("-")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();
                    return range_vec[0]..range_vec[1];
                })
                .collect::<Vec<_>>();
            return (ranges_vec[0].clone(), ranges_vec[1].clone());
        })
        .collect::<Vec<_>>();

    let num_ranges_consumed: i32 = ranges
        .iter()
        .map(|(range_a, range_b)| {
            (range_a.fully_contains(range_b) || range_b.fully_contains(range_a)) as i32
        })
        .sum();

    let num_ranges_overlap: i32 = ranges
        .iter()
        .map(|(range_a, range_b)| range_a.overlaps(range_b) as i32)
        .sum();

    println!("{:?}", ranges);
    println!("{}", num_ranges_consumed);
    println!("{}", num_ranges_overlap);
}

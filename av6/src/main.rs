use std::{collections::HashSet, fs};

struct U8BucketCounter {
    data: [u8; 256],
}

impl Default for U8BucketCounter {
    fn default() -> Self {
        Self { data: [0; 256] }
    }
}

impl U8BucketCounter {
    fn add(&mut self, c: u8) {
        self.data[c as usize] = self.data[c as usize].checked_add(1).unwrap();
    }

    fn remove(&mut self, c: u8) {
        self.data[c as usize] = self.data[c as usize].checked_sub(1).unwrap();
    }

    fn is_all_unique(&self) -> bool {
        self.data.iter().all(|&x| x <= 1)
    }
}

fn find_unique_fast(content: &str, num_unique: usize) -> Option<usize> {
    let mut bucket = U8BucketCounter::default();

    // feed the bucket the first window
    content.bytes().take(num_unique).for_each(|b| bucket.add(b));

    // if original happens to be unique return 0 + num_unique
    if bucket.is_all_unique() {
        return Some(num_unique);
    }

    for (index, window) in content.as_bytes().windows(num_unique + 1).enumerate() {
        let removed = window[0];
        let added = window[num_unique];

        bucket.remove(removed);
        bucket.add(added);

        if bucket.is_all_unique() {
            return Some(index + 1 + num_unique);
        }
    }

    return None;
}

fn find_unique(content: &str, num_unique: usize) -> Option<usize> {
    return content
        .as_bytes()
        .windows(num_unique)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == num_unique)
        .map(|pos| pos + num_unique);
}

fn main() {
    let file_path = "./data.txt";
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let unique_index = find_unique(&file_content, 4);

    println!("{}", file_content);
    println!("{}", unique_index.unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{find_unique, find_unique_fast};

    #[test]
    fn test_find_marker_4() {
        assert_eq!(Some(7), find_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(Some(5), find_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(Some(6), find_unique("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(
            Some(10),
            find_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
        );
        assert_eq!(Some(11), find_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }
    #[test]
    fn test_find_marker_fast_4() {
        assert_eq!(
            Some(7),
            find_unique_fast("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)
        );
        assert_eq!(Some(5), find_unique_fast("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(Some(6), find_unique_fast("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(
            Some(10),
            find_unique_fast("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
        );
        assert_eq!(
            Some(11),
            find_unique_fast("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)
        );
    }
    #[test]
    fn test_find_marker_14() {
        assert_eq!(Some(19), find_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(Some(23), find_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(Some(23), find_unique("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(
            Some(29),
            find_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
        );
        assert_eq!(
            Some(26),
            find_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)
        );
    }
}

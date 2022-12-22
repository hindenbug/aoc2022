use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");

    println!("{:?}, {:?}", part1(input), "")
}


fn part1(input: &str) -> usize {
    let mut packet: VecDeque<char> = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if packet.len() == 4 && all_unique(&packet) {
            return i;
        }

        packet.push_front(c);

        if packet.len() > 4  {
            packet.pop_back();
        }
    }

    panic!("No unique sequence found")
}

fn all_unique(input: &VecDeque<char>) -> bool {
    let mut chars = HashSet::new();
    for c in input {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }

    return true;
}


#[cfg(test)]
mod tests {
    use crate::part1;



    #[test]
    fn test_part1() {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    }

}

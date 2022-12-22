use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");

    println!("{:?}, {:?}", run(input, 4), run(input, 14))
}


fn run(input: &str, distinct: usize) -> usize {
    let mut packet: VecDeque<char> = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if packet.len() == distinct && all_unique(&packet) {
            return i;
        }

        packet.push_front(c);

        if packet.len() > distinct  {
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
    use crate::run;

    #[test]
    fn test_part1() {
        assert_eq!(7, run("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, run("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, run("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19, run("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14));
        assert_eq!(23, run("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, run("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    }
}

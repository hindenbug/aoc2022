use array_tool::vec::Intersect;


const CHARS: &str= "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = include_str!("input.txt");

    println!("{:?}, {:?}", part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
  let sum: u32 = input
       .lines()
       .map(|line| {
           let (first, last) = line.split_at(line.len() / 2);
           let p1 = first.chars().collect::<Vec<_>>();
           let p2 = last.chars().collect::<Vec<_>>();
           p1.intersect(p2)
       })
      .flatten()
      .map(|c| {
         CHARS.chars().into_iter().position(|r| r == c).unwrap() as u32 + 1 as u32
      })
      .sum();

    return sum
}

fn part2(input: &str) -> u32 {
    let sum: u32  = input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| {
            let p1 = group[0].chars().collect::<Vec<_>>();
            let p2 = group[1].chars().collect::<Vec<_>>();
            let p3 = group[2].chars().collect::<Vec<_>>();
            p1.intersect(p2).intersect(p3)
        })
        .flatten()
        .map(|c| {
            CHARS.chars().into_iter().position(|r| r == c).unwrap() as u32 + 1 as u32
        })
        .sum();

    return sum
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_part1() {
        assert_eq!(157, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(70, part2(INPUT));
    }
}

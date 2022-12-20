fn main() {
    let input = include_str!("input.txt");

    println!("{:?}, {:?}", part1(input), "")
}

fn part1(input: &str) -> u32  {
   return input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (to_pair(left), to_pair(right))
        })
        .filter(|(l, r)| {
            (l.0 >= r.0 && l.1 <= r.1) || (r.0 >= l.0 && r.1 <= l.1)
        })
       .count() as u32
}


fn to_pair(input: &str) -> (u32, u32) {
    let (left, right) = input.split_once('-').unwrap();
    (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(INPUT));
    }

    #[test]
    fn test_part2() {
    }
}

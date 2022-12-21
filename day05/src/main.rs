use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");

    println!("{:?}, {:?}", part1(input), "--")
}

type SupplyStacks = Vec<VecDeque<char>>;

#[derive(Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize
}

fn part1(input: &str) -> String {
    let (crates, moves) = input
        .split_once("\n\n").unwrap();

    let (left, right) = crates.rsplit_once("\n").unwrap();
    let stacks = right.split_whitespace().last().unwrap().parse().unwrap();

    let mut supply_stacks: SupplyStacks = vec![VecDeque::new(); stacks];

    for line in left.lines() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
           if c.is_alphabetic() {
               supply_stacks[i].push_back(c)
           }
        }
    }

    for line in moves.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let mov = Move {
            qty: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap()
        };

        for _ in 0..mov.qty {
            let c = supply_stacks[mov.from - 1].pop_front().unwrap();
            supply_stacks[mov.to - 1].push_front(c);
        }
    }

    return supply_stacks.iter().map(|v| v.front().unwrap()).collect();
}


#[cfg(test)]
mod tests {
    use crate::part1;

    const INPUT: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_part1() {
        assert_eq!("CMZ", part1(INPUT));
    }

}
fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input))
}

fn part1(input: &str) -> u32 {
    return input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
}

fn main() {
    let input = include_str!("../input.txt");

    let part_1 = do_part_1(&input);
    let part_2 = do_part_2(&input);

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

// a, x => rock
// b, y => paper
// c, z => scissors
fn do_part_1(input: &str) -> i32 {
    input.lines().map(|line|{
        match line {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => unimplemented!()
        }
    }).sum()
}

// a, 1 => rock
// b, 2 => paper
// c, 3 => scissors
// x => lose
// y => draw
// z => win
fn do_part_2(input: &str) -> i32 {
    input.lines().map(|line|{
        match line {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => unimplemented!()
        }
    }).sum()
}

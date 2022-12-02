fn main() {
    let input = include_str!("../input.txt");

    let part_1 = do_part_1(&input);
    let part_2 = do_part_2(&input);

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

// find the largest sum of consecutive numbers in the input.
// There is one number per line and these runs are separated by a 
// single blank line.
fn do_part_1(input: &str) -> i32 {
    let (_, max) = input.lines().fold((0, 0), |(sum, max), line| {
        match line.parse::<i32>() {
            Ok(value) => (sum + value, max),
            _ => (0, if sum > max { sum } else { max })
        }
    });
    max
}

fn do_part_2(input: &str) -> i32 {
    let (mut weights, _) =
        input.lines()
            .fold((Vec::<i32>::new(), 0), |(mut vec, sum), line| {
                match line.parse::<i32>() {
                    Ok(value) => (vec, sum + value),
                    _ => { vec.push(sum); (vec, 0) }
                }
            });
    weights.sort();
    weights.reverse();
    weights.iter().take(3).sum()
}

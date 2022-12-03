#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("../input.txt");

    let part_1 = do_part_1(&input);
    let part_2 = do_part_2(&input);

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

fn do_part_1(input: &str) -> i32 {
    input.lines()
        .map(|line| find_shared(line))
        .map(|shared_char| char_priority(shared_char))
        .sum()
}

fn do_part_2(input: &str) -> i32 {
    input.lines()
        .array_chunks()
        .map(|[f,s,t]| find_shared_in_group(f, s, t))
        .map(|chr| char_priority(chr))
        .sum()
}

fn find_shared(input: &str) -> char {
    let (right, left) = input.split_at(input.len() / 2);
    right.chars()
        .filter(|r| left.chars().any(|l| l == *r))
        .next().unwrap()
}

fn find_shared_in_group(first: &str, second: &str, third: &str) -> char {
    first.chars()
        .filter(|f| second.chars().any(|s| s == *f))
        .filter(|f| third.chars().any(|t| t == *f))
        .next().unwrap()
}

fn char_priority(input: char) -> i32 {
    match input {
        'a'..='z' => { (input as u8 - 'a' as u8 + 1) as i32 }
        'A'..='Z' => { (input as u8 - 'A' as u8 + 27) as i32 }
        _ => unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priorities() {
        assert_eq!(16, char_priority('p'));
        assert_eq!(38, char_priority('L'));
        assert_eq!(42, char_priority('P'));
        assert_eq!(22, char_priority('v'));
        assert_eq!(20, char_priority('t'));
        assert_eq!(19, char_priority('s'));
    }

    #[test]
    fn test_find_shared_in_group() {
        let f = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let s = "ttgJtRGJQctTZtZT";
        let t = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!('Z', find_shared_in_group(f, s, t))
    }

    #[test]
    fn shared_item_pack_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let shared = find_shared(&input);
        assert_eq!(shared, 'p');
    }

    #[test]
    fn shared_item_pack_2() {
        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let shared = find_shared(&input);
        assert_eq!(shared, 'L');
    }

    #[test]
    fn shared_item_pack_3() {
        let input = "PmmdzqPrVvPwwTWBwg";
        let shared = find_shared(&input);
        assert_eq!(shared, 'P');
    }

    #[test]
    fn shared_item_pack_4() {
        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let shared = find_shared(&input);
        assert_eq!(shared, 'v');
    }

    #[test]
    fn shared_item_pack_5() {
        let input = "ttgJtRGJQctTZtZT";
        let shared = find_shared(&input);
        assert_eq!(shared, 't');
    }

    #[test]
    fn shared_item_pack_6() {
        let input = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let shared = find_shared(&input);
        assert_eq!(shared, 's');
    }
}

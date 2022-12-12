use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    let part_1 = calculate_start_of_packet_idx(input);
}

fn calculate_start_of_packet_idx(input: &str) -> usize {
    let mut window = VecDeque::new();
    let line = input.lines().next().unwrap();
    line.char_indices().reduce()
}

#[cfg(test)]
mod test {
    use crate::calculate_start_of_packet_idx;

    #[test]
    fn test_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker_idx = calculate_start_of_packet_idx(input);
        assert_eq!(5, marker_idx);
    }

    #[test]
    fn test_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker_idx = calculate_start_of_packet_idx(input);
        assert_eq!(6, marker_idx);
    }

    #[test]
    fn test_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker_idx = calculate_start_of_packet_idx(input);
        assert_eq!(10, marker_idx);
    }

    #[test]
    fn test_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker_idx = calculate_start_of_packet_idx(input);
        assert_eq!(11, marker_idx);
    }
}

use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    let part_1 = calculate_start_of_packet_idx(input, 4);
    let part_2 = calculate_start_of_packet_idx(input, 14);

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

fn calculate_start_of_packet_idx(input: &str, window_length: usize) -> usize {
    let line = input.lines().next().unwrap();
    let mut win = VecDeque::<(usize, char)>::new();
    for (idx, chr) in line.chars().enumerate().into_iter() {
        // deal with runtime length
        if win.len() >= window_length {
            win.pop_front();
        }
        win.push_back((idx, chr));
        // matching
        if win.len() == window_length {
            let mut multiple_found = false;
            for item_idx in 0..window_length {
                for check_idx in (item_idx + 1)..window_length {
                    multiple_found = multiple_found
                        || win[item_idx].1 == win[check_idx].1
                }
            }
            if !multiple_found { return idx + 1; }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use crate::calculate_start_of_packet_idx;

    #[test]
    fn test_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker_idx = calculate_start_of_packet_idx(input, 4);
        assert_eq!(5, marker_idx);
    }

    #[test]
    fn test_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker_idx = calculate_start_of_packet_idx(input, 4);
        assert_eq!(6, marker_idx);
    }

    #[test]
    fn test_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker_idx = calculate_start_of_packet_idx(input, 4);
        assert_eq!(10, marker_idx);
    }

    #[test]
    fn test_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker_idx = calculate_start_of_packet_idx(input, 4);
        assert_eq!(11, marker_idx);
    }
}

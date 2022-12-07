use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug)]
struct Move {
    quantity: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn new(quantity: u32, from: u32, to: u32) -> Self {
        Self { quantity, from, to }
    }
}

type State = HashMap<u32, Vec<char>>;

fn main() {
    let input = include_str!("../input.txt");

    let (initial_state, move_list) = parse_input(input);

    let part_1 = do_part_1(initial_state.clone(), &move_list);
    let part_2 = do_part_2(initial_state.clone(), &move_list);

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

fn do_part_2(mut state: State, moves: &Vec<Move>) -> String {
    for step in moves {
        apply_move_step_9001(&mut state, step);
    }
    get_top_crates(state)
}

fn do_part_1(mut state: State, moves: &Vec<Move>) -> String {
    for step in moves {
        apply_move_step_9000(&mut state, step);
    }
    get_top_crates(state)
}

fn get_top_crates(state: State) -> String {
    let mut keys: Vec<&u32> = state.keys().collect();
    keys.sort();
    let mut top_crates = String::new();
    for key in keys {
        top_crates.push(*state.get(key).unwrap().last().unwrap());
    }
    top_crates
}

fn apply_move_step_9001(state: &mut State, step: &Move) {
    let tail = {
        let from = state.get_mut(&step.from).unwrap();
        let split_idx = from.len() - step.quantity as usize;
        from.split_off(split_idx)
    };
    state.get_mut(&step.to).unwrap().extend_from_slice(&tail);
}

fn apply_move_step_9000(state: &mut State, step: &Move) {
    for _ in 0..step.quantity {
        let item = state.get_mut(&step.from).unwrap().pop().unwrap();
        state.get_mut(&step.to).unwrap().push(item);
    }
}


fn parse_input(input: &str) -> (State, Vec<Move>) {
    let mut lines = input.lines();
    let state = parse_state(&mut lines);
    let moves = parse_moves(&mut lines);
    (state, moves)
}

fn parse_moves(lines: &mut Lines) -> Vec<Move> {
    lines.map(|line| {
        let entries: Vec<&str> = line.split(' ').collect();
        let quantity = entries[1].parse::<u32>().unwrap();
        let from = entries[3].parse::<u32>().unwrap();
        let to = entries[5].parse::<u32>().unwrap();
        Move::new(quantity, from, to)
    }).collect()
}

fn parse_state(lines: &mut Lines) -> State {
    let mut state = HashMap::new();
    let mut state_lines = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" { break; }
        state_lines.push(parse_state_line(line))
    }
    state_lines.reverse();

    let mut state_lines_iter = state_lines.into_iter();

    // state - keys
    for entry in state_lines_iter.next().unwrap() {
        state.insert(entry.to_digit(10).unwrap(), Vec::new());
    }

    // state - values
    while let Some(entries) = state_lines_iter.next() {
        for (idx, val) in entries.iter().enumerate() {
            let stack = state.get_mut(&((idx + 1) as u32)).unwrap();
            if val != &' ' {
                stack.push(*val);
            }
        }
    }

    state
}

fn parse_state_line(line: &str) -> Vec<char> {
    let len = (line.len() + 1) / 4;
    let mut chars = line.chars();
    let mut entries = Vec::new();
    for idx in 0..len {
        let n = if idx == 0 { 1 } else { 3 };
        entries.push(chars.nth(n).unwrap())
    }
    entries
}

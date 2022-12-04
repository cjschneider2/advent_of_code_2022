fn main() {
    let input = include_str!("../input.txt");

    let part_1 = do_part_1(&input);
    let part_2 = do_part_2(&input);

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

#[derive(Debug)]
struct Span {
    lower: i32,
    upper: i32,
}

impl Span {
    fn new_from_str_pair(input: (&str, &str)) -> Self {
        Self {
            lower: (input.0).parse::<i32>().unwrap(),
            upper: (input.1).parse::<i32>().unwrap(),
        }
    }

    fn contains_span(&self, other: &Span) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn has_overlap_with(&self, other: &Span) -> bool {
        self.lower <= other.lower && self.upper >= other.lower
            || self.lower <= other.upper && self.upper >= other.upper
    }
}

fn parse_spans_from_input(input: &str) -> Vec<(Span, Span)> {
    input.lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(span_1, span_2)| {
            (Span::new_from_str_pair(span_1.split_once('-').unwrap()),
             Span::new_from_str_pair(span_2.split_once('-').unwrap()))
        })
        .collect()
}

fn do_part_1(input: &str) -> i32 {
    let spans = parse_spans_from_input(input);
    spans.iter()
        .map(|(left, right)| {
            (left.contains_span(right) || right.contains_span(left)) as i32
        })
        .sum()
}

fn do_part_2(input: &str) -> i32 {
    let spans = parse_spans_from_input(input);
    spans.iter()
        .map(|(left, right)| {
            (left.has_overlap_with(right) || right.has_overlap_with(left)) as i32
        })
        .sum()
}

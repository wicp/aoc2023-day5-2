use std::str::Lines;

fn parse_ints(input: &str) -> Vec<u32> {
    input.split_whitespace().map(|seed| seed.parse().unwrap()).collect()
}

fn parse_map(input_lines: &mut Lines<'_>) -> Vec<(u32,u32,u32)> {
    let mut map: Vec<(u32, u32, u32)> = vec![];
    while let Some(line) = input_lines.next() { 
        if line.is_empty() { break };
        let [destination, source, range] = parse_ints(line)[0..3] else { panic!("invalid line {}", line) };
        map.push((destination, source, range));
    }
   map 
}

fn apply_map(input: u32, map: &Vec<(u32,u32,u32)>) -> u32 {
    for (destination, source, range) in map {
        if *source <= input && input <= *source+*range {
            return *destination+(input-*source)
        }
    }
    input
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Could not read input.txt in current directory");
    let mut input_lines = input.lines();
    let seeds = parse_ints(&input_lines.next().unwrap()[7..]);
    let mut maps = vec![];
    input_lines.next();
    for _ in 0..7 {
        input_lines.next();
        maps.push(parse_map(&mut input_lines));
    }
    let locations = seeds
        .into_iter()
        .map(|seed|
             maps.clone().into_iter().fold(seed, |acc, map| apply_map(acc, &map)));
    println!("{}", locations.min().unwrap());
}

use std::collections::{HashSet, HashMap};

static DAY4: &str = include_str!("../data/day4.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut points: usize = 0;

    for line in DAY4.split('\n') {

        let winning_numbers : HashSet<&str>  = HashSet::from_iter(line[9..40].split_whitespace());
        let game_numbers : HashSet<&str>  = HashSet::from_iter(line[41..].split_whitespace());
        let winning = winning_numbers.intersection(&game_numbers);
        let count = winning.count() as u32;

        if count > 0 {
            points += usize::pow(2,count -1) ;
        }

    }
    points
}

fn part2() -> usize {
    let mut cards_total: usize = 0;

    let mut copies : HashMap<usize,usize> = HashMap::new();

    for (i, line) in DAY4.split("\n").enumerate() {

        let winning_numbers : HashSet<&str>  = HashSet::from_iter(line[9..40].split_whitespace());
        let game_numbers : HashSet<&str>  = HashSet::from_iter(line[41..].split_whitespace());
        let winning_count = winning_numbers.intersection(&game_numbers).count();
        let current_copies  = *copies.get(&i).unwrap_or(&0);

        cards_total += 1 + current_copies;

        for copy  in 0..winning_count {
            *copies.entry(i+1+copy).or_insert(0) += 1 + current_copies;
        }
    }
    cards_total
}

use std::cmp::min;
use regex::Regex;

static DAY3 : &str = include_str!("../data/day3.txt");

fn filter(lines : &[&str]) -> Vec<u64> {
    let re =  Regex::new(r"(\d+)").unwrap();

    let numbers: Vec<u64> =  re.find_iter(lines[1]).filter_map(|m| {
        let mut checkstr : String = String::new();
        let range = m.start().saturating_sub(1) .. min(m.end()+1, 140);

        checkstr  += lines[0].get(range.clone()).unwrap_or("");
        checkstr  += lines[1].get(range.clone()).unwrap_or("");
        checkstr  += lines[2].get(range).unwrap_or("");

        if checkstr.chars().any(|c| !c.is_ascii_digit() && c!= '.'){
            m.as_str().parse::<u64>().ok()
        }
        else {
            None
        }
    }).collect();
    numbers
}


fn main() {

    let pad =  ".".repeat(140);
    let mut lines :Vec<&str> = DAY3.split('\n').collect();
    lines.insert(0, &pad[..]);
    lines.insert(lines.len(), &pad[..]);
    let sum = lines.windows(3).flat_map(filter).sum::<u64>();

    println!("{}", sum);

    
    let mut gears: Vec<(usize, usize)> =  Vec::new();
    for (row, &line) in lines.iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == '*' {
                gears.push((row, col));
            }
        }
    }

    for (row, col) in gears {
        let numbers : Vec<u64> = Vec::new();

        // handle over
        let upper_left = lines[row-1].chars().nth(col-1).is_some_and(|t| t.is_ascii_digit());
        let upper_center = lines[row-1].chars().nth(col).is_some_and(|t| t.is_ascii_digit());
        let upper_right = lines[row-1].chars().nth(col+1).is_some_and(|t| t.is_ascii_digit());

        let left = lines[row].chars().nth(col-1).is_some_and(|t| t.is_ascii_digit());
        let right = lines[row].chars().nth(col+1).is_some_and(|t| t.is_ascii_digit());

        let lower_left = lines[row+1].chars().nth(col-1).is_some_and(|t| t.is_ascii_digit());
        let lower_center = lines[row+1].chars().nth(col).is_some_and(|t| t.is_ascii_digit());
        let lower_right = lines[row+1].chars().nth(col+1).is_some_and(|t| t.is_ascii_digit());

    }


}
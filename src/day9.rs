//use rayon::prelude::*;

static DAY9: &str = include_str!("../data/day9.txt");

fn make_predmap(line : &str) -> Vec<Vec<i64>> {
    let mut predictions : Vec<Vec<i64>> = Vec::new();
    predictions.push(line.split_whitespace().map(|d| d.parse::<i64>().unwrap()).collect());
    loop {
        let next : Vec<i64> = predictions.last().unwrap().windows(2).map(|v| v[1]- v[0] ).collect();
        if next.iter().filter(|&&x| x==0).count() == next.len() {
            break;
        }
        predictions.push(next);
    }
    predictions
}

fn predict_next(line : &str) -> i64 {
    let predictions = make_predmap(line);
    predictions.iter().rfold(0, |sum,f| f.last().unwrap() + sum )
}

fn predict_prev(line : &str) -> i64 {
    let predictions = make_predmap(line);
    predictions.iter().rfold(0, |sum,f| f.first().unwrap() - sum )
}

fn main() {
    println!("Part 1: {}",DAY9.split('\n').map(predict_next).sum::<i64>());
    println!("Part 2: {}",DAY9.split('\n').map(predict_prev).sum::<i64>());
}
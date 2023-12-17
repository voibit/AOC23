use std::{str::SplitWhitespace, collections::HashMap, iter, cmp::Ordering};

static DAY7: &str = include_str!("../data/day7.txt");

#[derive(Eq)]
#[derive(Debug)]
pub struct Game {
    hand : [usize;5],
    bet : usize,
    strength : usize
}

enum Strenth{
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl From<Strenth> for usize {
    fn from(c: Strenth) -> Self {
        match c {
            Strenth::High => 1,
            Strenth::OnePair => 2,
            Strenth::TwoPair => 3,
            Strenth::ThreeOfAKind => 4,
            Strenth::FullHouse => 5,
            Strenth::FourOfAKind => 6,
            Strenth::FiveOfAKind => 7,
        }
    }
}


impl Game {
    pub fn new(line :&str) -> Self {

        let segments : Vec<&str> = line.split_whitespace().collect();
        let bet = segments[1].parse::<usize>().unwrap();
        let mut hand : [usize; 5] =  [0,0,0,0,0];
    
        let map: &[char; 13] = &['2','3','4','5','6','7','8','9','T','J','Q','K','A'];

        for (i, s) in segments[0].chars().enumerate() {
            hand[i] = map.iter().position(|&r| r == s).unwrap();
        }

        let mut s = Strenth::High;

        for i in 0..13 {
            match (&s, hand.iter().filter(|&&x| x==i).count()) {
                (_, 5) => {s = Strenth::FiveOfAKind; break}
                (_, 4) => {s = Strenth::FourOfAKind; break}
                (Strenth::OnePair,3) => {s = Strenth::FullHouse; break},
                (Strenth::High, 3) => s = Strenth::ThreeOfAKind,
                (Strenth::OnePair,2) => s = Strenth::TwoPair,
                (Strenth::High,2) => s = Strenth::OnePair,
                (Strenth::ThreeOfAKind,2) => {s = Strenth::FullHouse; break},
                _ => ()
            };
        }
        Self{hand,bet, strength: s as usize}

    }


    pub fn new2(line :&str) -> Self {

        let segments : Vec<&str> = line.split_whitespace().collect();
        let bet = segments[1].parse::<usize>().unwrap();
        let mut hand : [usize; 5] =  [0,0,0,0,0];
    
        let map: &[char; 13] = &['J','2','3','4','5','6','7','8','9','T','Q','K','A'];

        for (i, s) in segments[0].chars().enumerate() {
            hand[i] = map.iter().position(|&r| r == s).unwrap();
        }

        let mut s = Strenth::High;

        for i in 1..13 {
            match (&s, hand.iter().filter(|&&x| x==i).count()) {
                (_, 5) => {s = Strenth::FiveOfAKind; break}
                (_, 4) => {s = Strenth::FourOfAKind; break}
                (Strenth::OnePair,3) => {s = Strenth::FullHouse; break},
                (Strenth::High, 3) => s = Strenth::ThreeOfAKind,
                (Strenth::OnePair,2) => s = Strenth::TwoPair,
                (Strenth::High,2) => s = Strenth::OnePair,
                (Strenth::ThreeOfAKind,2) => {s = Strenth::FullHouse; break},
                _ => ()
            };
        }

        match (&s, hand.iter().filter(|&&x| x==0).count()) {
            (_,0) => (),
            (_, 5) | (Strenth::FourOfAKind, 1) => s= Strenth::FiveOfAKind,

            (Strenth::High, 1) => s= Strenth::OnePair, 
            (Strenth::High, 2) => s= Strenth::ThreeOfAKind, 
            (Strenth::High, 3) => s= Strenth::FourOfAKind, 
            (Strenth::High, 4) => s= Strenth::FiveOfAKind, 

            (Strenth::OnePair, 1) => s = Strenth::ThreeOfAKind, 
            (Strenth::OnePair, 2) => s = Strenth::FourOfAKind,
            (Strenth::OnePair, 3) => s = Strenth::FiveOfAKind, 

            (Strenth::TwoPair, 1) => s = Strenth::FullHouse, 

            (Strenth::ThreeOfAKind,1) => s = Strenth::FourOfAKind,
            (Strenth::ThreeOfAKind,2) => s = Strenth::FiveOfAKind,
            _ => ()
        }
        Self{hand,bet, strength: s as usize}

    }


    
}


impl Ord for Game {
    fn cmp(&self, other: &Game) -> Ordering {
        if self.strength != other.strength {
            self.strength.cmp(&other.strength)
        }
        else {
            for (ai, bi) in self.hand.iter().zip(other.hand.iter()) {
                match ai.cmp(&bi) {
                    Ordering::Equal => continue,
                    ord => return ord
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Game) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        self.strength == other.strength
    }
}

fn main() {

    let mut games : Vec<Game> =  DAY7.split('\n').map(Game::new).collect();
    games.sort();

    let sum : usize = games.iter().enumerate().map(|(i, game) | (i+1)* game.bet ).sum();

    println!("{}", sum);



}
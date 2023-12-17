use std::{collections::HashSet, fmt};
use crate::Direction::*;
static DAY16: &str = include_str!("../data/day16.txt");


#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down 
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Beam {
    pub direction : Direction,
    pub position : (usize, usize)
}

impl fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.direction, self.position)
    }
}

struct Map {
    tiles : [[char;110] ; 110],
    visited_tiles : [[bool;110] ; 110],
    finished_beams: HashSet<Beam>,
    beam_queue: HashSet<Beam>,
}

impl Map  {
    fn new(lines :  &str) ->Self {
        let mut tiles : [[char;110] ; 110] =  [['.';110]; 110];
        lines.split('\n').zip(tiles.iter_mut()).for_each(|(from,to)|{
            from.chars().zip(to.iter_mut()).for_each(|(fc, tc)| *tc=fc);
        });

        Self{tiles,
            visited_tiles: [[false; 110];110],
            finished_beams:HashSet::new(),
            beam_queue:HashSet::new(),
        }
    }
    fn clear(&mut self) {
        self.visited_tiles = [[false;110] ; 110];
        self.finished_beams.clear();
        self.beam_queue.clear();
    }

    fn start(& mut self, beam : Beam ) {
        self.clear();
        self.beam_queue.insert(beam);

        while let Some(beam) = self.next_beam() {
            self.calc_beam(beam);
        }
    }

    fn next_beam(&self) -> Option<Beam> {
        self.beam_queue.difference(&self.finished_beams).next().cloned()
    }

    fn next_pos(&mut self, beam: &mut Beam) -> Option<()> {

        let  (mut row, mut column) = beam.position;

        match (&beam.direction, self.tiles.get(row).and_then(|l| l.get(column))?) {   
            (Right, '/') | (Left, '\\') => beam.direction = Up,
            (Left, '/') | (Right, '\\') => beam.direction = Down,
            (Down, '/') | (Up, '\\')    => beam.direction = Left,
            (Up, '/') | (Down, '\\')    => beam.direction = Right,

            (Up | Down, '-') => {
                if let Some(column) = column.checked_sub(1){
                    self.beam_queue.insert(Beam{direction:Left, position:(row,column)});
                }
                if column < 109 {
                    self.beam_queue.insert(Beam{direction:Right, position:(row,column+1)});
                }
                return None;
            }
            (Left | Right, '|') => {
                if let Some(row) = row.checked_sub(1){
                    self.beam_queue.insert(Beam{direction:Up, position:(row,column)});
                }
                if row < 109 {    
                    self.beam_queue.insert(Beam{direction:Down, position: (row+1,column)});
                }
                return None;
            }
            _ => ()
        }

        match beam.direction {
            Right   => column+=1,
            Left    => column = column.checked_sub(1)?,
            Up      => row = row.checked_sub(1)?,
            Down    => row+=1,
        }

        if row >= 110 || column >= 110 {
            return None;
        }
        beam.position = (row, column);

        Some(())

    }

    fn calc_beam(&mut self, mut beam: Beam) {
        self.finished_beams.insert(beam.clone());
        self.visited_tiles[beam.position.0][beam.position.1] = true;

        while self.next_pos(&mut beam).is_some() {
            self.visited_tiles[beam.position.0][beam.position.1] = true;
        }
    }

    fn count_energized(&self) -> usize {
        self.visited_tiles.iter().flatten().filter(|&&tile| tile).count()
    }

}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..110 {
            for column in 0..110 {
                write!(f, "{}", if self.visited_tiles[row][column] {'#'} else {'.'} )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut map = Map::new(DAY16);
    map.start(Beam{direction:Right, position:(0,0)});
    println!("Part1: {}", map.count_energized());

    let mut max : Vec<usize> = Vec::new();

    for i in 0..110 {
        // Beams from left going right
        map.start(Beam{direction:Right, position:(i,0)});
        max.push(map.count_energized());

        // Beams from top going down
        map.start(Beam{direction:Down, position:(0,i)});
        max.push(map.count_energized());

        // Beams from right going left
        map.start(Beam{direction:Left, position:(i,109)});
        max.push(map.count_energized());

        // Beams from bottom going up
        map.start(Beam{direction:Up, position:(109,i)});
        max.push(map.count_energized());
    }
    
    println!("Part2: {}", max.iter().max().unwrap());

}
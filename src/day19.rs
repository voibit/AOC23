use std::collections::HashMap;



static DAY19: &str = include_str!("../data/day19.txt");

#[derive(Debug)]
pub enum Rules<'a> {
    Less(char, usize, &'a str),
    Greater(char, usize, &'a str),
    Direct(&'a str),
}

impl<'a> Rules<'a> {
    fn parse(rule :&'a str ) -> Self {
        if let Some(dest_idx) = rule.find(':') {
            let var : char  = rule.chars().nth(0).unwrap();
            let value : usize  = rule[2..dest_idx].parse().unwrap();
            if rule.chars().nth(1).is_some_and(|c: char| c=='<') {
                return Rules::Less(var, value, &rule[dest_idx+1..rule.len()]);
            }
            if rule.chars().nth(1).is_some_and(|c| c=='>') {
                return Rules::Greater(var,value, &rule[dest_idx+1..rule.len()]);
            }
        } 
        Rules::Direct(rule)
    }
}

struct Workflows<'a> {
    workflows : HashMap<&'a str,Vec<Rules<'a>>>,
    rating_map : HashMap<char, usize>,
    ratings : Vec<[usize; 4]>

}

impl<'a> Workflows<'a>  {
    fn new(lines : &'a str) -> Self {
        let mut workflows : HashMap<&str,Vec<Rules>> = HashMap::new();
        let rating_map : HashMap<char, usize> = HashMap::from([('x',0),('m',1), ('a',2), ('s',3)]);
        let mut ratings : Vec<[usize; 4]> = Vec::new();
    
        for line  in lines.lines() { 
            if !line.starts_with('{') {
                let Some(name_idx) = line.find('{') else {continue;};
                let workflow_name =  &line[0..name_idx];
                let workflow_rules =  &line[name_idx+1..line.len()-1];
                let vec : Vec<Rules> = workflow_rules.split(',').map(Rules::parse).collect();
                workflows.insert(workflow_name, vec);
            }
            else {
                let mut rating : [usize; 4] = [0; 4];
                for (i, val) in line[1..line.len()-1].split(',').take(4).enumerate() {
                    rating[i] = val[2..val.len()].parse().unwrap();
                }
                ratings.push(rating);
            }
        }

        Self{workflows,rating_map,ratings}
    }

    fn parse(&self) -> (usize, usize) {

        let (mut accepted, mut rejected)  =(0usize,0usize);
        let mut rating_iter = self.ratings.iter();
        let mut rating = rating_iter.next();
        let mut dest = "in";
        
        while let Some(r) = rating {
            for rule in self.workflows.get(dest).unwrap() {
                //println!("\tRule: {:?}",rule);
                dest = match &rule {
                    Rules::Less(c, val, new_dest) => {
                        if r[self.rating_map[c]] < *val {
                            new_dest
                        } else {continue;}
                    },
                    Rules::Greater(c, val, new_dest) => {
                        if r[self.rating_map[c]] > *val {
                            new_dest
                        } else {continue;}
                    },
                    Rules::Direct(new_dest) => new_dest
                };

                match dest {
                    "A" => {
                        accepted+=r.iter().sum::<usize>();
                        rating = rating_iter.next();
                        dest = "in";
                    }
                    "R" => {
                        rejected+=1;
                        rating = rating_iter.next();
                        dest = "in";
                    }
                    _ => ()
                }
                break;
            }
        }
        (accepted, rejected)
    }

    
}
    

fn main() {
    let workflows = Workflows::new(DAY19);

    println!("{:?}", workflows.parse());

}
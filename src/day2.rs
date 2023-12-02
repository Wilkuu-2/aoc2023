use regex::Regex; 

struct Hand {
    red: usize, 
    green: usize, 
    blue: usize
}

impl Clone for Hand {
    fn clone(&self) -> Self {
        Self {red: self.red, green: self.green, blue: self.blue} 
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"[{}]",self)
    }
}
impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"r: {}, g: {}, b: {}", self.red, self.green ,self.blue)
    }
}
#[derive(Debug)]
struct ParseError(String); 
impl<'a> std::error::Error for ParseError {}
impl<'a> std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"[{}]",self.0)
    }
}


impl std::str::FromStr for Hand {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut out = Self::new(0,0,0); 
        for val in input.split(",") {
            let mut num: u32  = 0; 
            let mut color: &str = ""; 
            for (i, c) in val.char_indices() {
                // Parse the number
                if let Some(d) = c.to_digit(10) {
                    num = num * 10 + d;
                // Get a slice containing the string for color
                } else if c.is_alphabetic(){
                    color = &val[i..val.len()];
                    break
                } 
            }
            // println!("{val} > {num} {color}");
            let count: usize = num.try_into().unwrap();
            match color {
                "blue" => out.blue = count,
                "red" =>  out.red = count, 
                "green" => out.green = count, 
                _ => return Err(ParseError(format!("Unmatched color {}", color))),
            } 
        } 
        Ok(out)
    }
}

impl Hand {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self{red,green,blue}
    } 
    fn fits_in(&self, max: &Self) -> bool{
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue 
    }

    fn make_fit(&mut self, to_fit: &Self) {
        self.red = self.red.max(to_fit.red);
        self.green = self.green.max(to_fit.green);
        self.blue = self.blue.max(to_fit.blue);
    }

    fn power(&self) -> usize {
        self.red * self.blue * self.green
    } 

} 

struct Game { 
    id: usize,
    hands: Vec<Hand>
}

impl std::str::FromStr for Game {
    type Err = ParseError; 
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let id_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
        if let Some(capture) = id_re.captures(line) {
            let (_, [id_str, state_str]) = capture.extract();
            // println!("{} >> {}", id_str, state_str);
            let id: usize = id_str.parse().unwrap_or(0);
            let hands: Vec<Hand> = state_str.split(";").map(|s| s.parse().unwrap()).collect(); 
            Ok(Game {id, hands})
        } else {
            Err(ParseError(format!("Can't parse: \n {} \n is invalid syntax", line))) 
        }
    }
}

impl Game {

    fn get_minimum_cubes(&self) -> Hand{
        let mut min = Hand::new(0,0,0);
        for hand in self.hands.iter(){
           min.make_fit(hand); 
        }
        min
    } 

    fn validate_against(&self, maximums: Hand) -> bool{
         for hand in self.hands.iter() {
            if !hand.fits_in(&maximums) {
                // println!("{} > {} no fit", self.id, hand);
                return false; 
            } 
         } 
         //println!("{} > fits:\n {:?}", self.id, self.hands);
         // println!("{} > fit", self.id);
         true 

    }
} 

const TEST_1: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn part1(input: &str, max: Hand) -> usize {
    input.lines()
        .into_iter()
        .map(|x| x.parse::<Game>().unwrap())
        .filter(|x| x.validate_against(max.clone()))
        .map(|x| x.id)
        .sum()
} 

fn part2(input: &str) -> usize {
    input.lines()
        .into_iter()
        .map(|x| {
            let game: Game = x.parse().unwrap(); 
            let pow = game.get_minimum_cubes().power();
            //println!("{} -> {}", game.id, pow);
            pow
        }).sum()
} 

const INPUT: &str = include_str!("../inputs/day2.txt");

fn main() {
   println!("[-------------- PART 1 --------------]") ;
   println!("[TEST]: {}", part1(TEST_1,Hand::new(12,13,14)));
   println!("[OUTP]: {}", part1(INPUT,Hand::new(12,13,14)));
   println!("[-------------- PART 2 --------------]") ;
   println!("[TEST]: {}", part2(TEST_1));
   println!("[OUTP]: {}", part2(INPUT));

} 

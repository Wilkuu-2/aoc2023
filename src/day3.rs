#[allow(unused)]
// Advent of code Day 3

const INPUT: &str = include_str!("./../inputs/day3.txt");
const TEST_1: &str = 
r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
....*....*
.664.598.."; 

const SYMS: &[char] = &['*', '@', '/', '+', '$', '=', '&', '-', '#', '%'] ; 

const TEST_TRUE: &str =
r"..22..
....*.
.1111.
......
";


fn find_syms(input: &str) -> Vec<char> {
    let mut out = Vec::new();
    for x in input.chars() {
       if !x.is_digit(10) && x != '.' && !x.is_whitespace() {
            if !out.contains(&x) {
                out.push(x); 
            }  
       } 
    } 
    out
} 

#[derive(Debug)]
struct RectLine<'a>{
    top: Option<&'a str>,
    mid: &'a str,
    bot: Option<&'a str>,
    center_local: usize,
    position: (usize, usize),
    hsize: usize,
}

impl RectLine<'_> {
    fn is_comp(&self) -> bool {
        let mut comp: bool = false; 
        for c in SYMS {
            let t = match self.top {
                Some(t) => t.contains(*c),
                None => false
            };
            let m = self.mid.contains(*c);
            let b = match self.bot {
              Some(b) => b.contains(*c),
              None => false
            };
            if t || m || b {
              comp= true; 
            }
        }
        
        let t = match self.top {Some(i) => i, None => ""}; 
        let b = match self.bot {Some(i) => i, None => ""}; 
        println!("{}\n{} => {} => {} \n{}\n",t,self.mid, comp, self.get_core_number() ,b);
        comp 
    }

    fn get_core_number(&self) -> usize {
        let n = &self.mid[self.center_local..self.center_local+self.hsize];
        let v: usize = n.parse().unwrap();

        v
    } 
} 

#[derive(Debug)]
struct Blueprint{
    lines : Vec<&'static str>,
    width: usize
}

impl Blueprint{
    fn new(input: &'static str) -> Self{
        let lines: Vec<&str> = input.lines().collect();
        let width: usize = lines.iter().map(|x| x.chars().count()).max().unwrap(); 

        Self { lines, 
               width}
    }

    fn get_rectline(&self, cx: usize, cy: usize, l: usize) -> RectLine<'_>
    {
        let lx: usize = match cx {
           0 => 0, 
           _ => cx - 1,
        }; 

        let mut rx: usize = cx + l + 1;

        if rx > self.width { 
            rx = self.width  
        }

        let mut top: Option<&str> = None;
        if cy > 0 { 
            // Manual bounds check for the subraction
            top = match self.lines.get(cy-1) {
                Some(t) => Some(&t[lx..rx]),
                None => None 
            } 
            
        } 
        // No needs for manual bounds checking 
        let bot: Option<&str> = match self.lines.get(cy+1).copied() {
            Some(b) => Some(&b[lx..rx]),
            None => None 
        };

        let mid: &str = &self.lines.get(cy).copied().unwrap()[lx..rx];
        
        RectLine {
            top,
            mid,
            bot, 
            center_local: cx - lx,
            position: (cx,cy),
            hsize: l,
        } 
         
    }

    fn get_all_numbers(&self) -> Vec<RectLine<'_>>{
        let mut out: Vec<RectLine<'_>> = Vec::new(); 
        for y in 0..self.lines.len(){
           let l = self.lines.get(y).unwrap(); // all the values of i are in bounds 
           let mut c_iter = l.char_indices().into_iter();
           let mut num_begin: i64 = -1; 
           while let Some((x, c)) = c_iter.next(){
               if c.is_digit(10){
                    if num_begin < 0 {
                        num_begin = x.try_into().unwrap(); 
                    } 
               } else if num_begin > -1 {
                    let cx: usize = num_begin.try_into().unwrap();
                    out.push(self.get_rectline(cx, 
                                               y, 
                                               x - cx));
                    num_begin = -1; 
               } 
           }
        }
        out 
            
    }

}

fn part1(bp: &Blueprint) -> usize {
    let a: Vec<usize>= bp.get_all_numbers()
        .iter()
        .filter_map(|x|{ 
            match x.is_comp() {
                true => Some(x.get_core_number()), 
                false => None
            }
        }).collect();

   a.iter().sum() 
    
} 

fn main(){
    assert_eq!(part1(&Blueprint::new(TEST_TRUE)), 1133);
    print!("aaaaaaa");

    // let syms = find_syms(INPUT); 
    // println!("[SYMBOLS]:\n{:?}", syms);
    /* let test_bp = Blueprint::new(TEST_1);
    let nums = test_bp.get_all_numbers();
    // print!("{:?}", nums);
    let is_comps = nums.iter().filter(|x| x.is_comp()).count();
    assert_eq!(nums.len(), 10);
    assert_eq!(is_comps, 8);
    assert_eq!(part1(&test_bp), 4361); */

    let bp = Blueprint::new(INPUT); 
    println!("[PART 1]: {}", part1(&bp))
    
}

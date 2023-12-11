use itertools::Itertools; 

const TEST_1: &str = 
r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

const INPUT: &str = include_str!("../inputs/day11.txt");

struct Universe<'a> {
    rows: Vec<UniverseLine>, 
    cols: Vec<UniverseLine>, 
    galaxies: Vec<Galaxy>,
    _data:  &'a str  

} 

struct UniverseLine{
    original_z: usize,
    expanded_z: usize,

} 

struct Galaxy{
    column: usize, 
    row: usize  
}

impl<'a> Universe<'a> {
    fn new(data: &'a str) -> Self {
       let mut out = Universe { rows: Vec::new(), cols: Vec::new(), galaxies: Vec::new(), _data: data };
       let data_raw: Vec<&str> = data.lines().collect(); 
       let width = data_raw.get(0).unwrap().len();
       
       for x in 0..width {
            out.cols.push(UniverseLine{original_z:x, expanded_z:x});
       } 

       for y in 0..data_raw.len(){
           out.rows.push(UniverseLine{original_z:y, expanded_z:y});
           for x in 0..width{
               let c: char = data_raw.get(y).unwrap().chars().nth(x).unwrap();
               if c == '#' {
                   out.galaxies.push(Galaxy {column: x, row: y} )
               }  
           }
       }
       out 
    } 

    fn expand(&mut self, distance: usize){
        let mut row_exp: usize = 0;
        let mut col_exp: usize = 0; 

        let occupied_rows: Vec<_> = self.galaxies.iter().map(|x| x.row).unique().collect();
        let occupied_cols: Vec<_> = self.galaxies.iter().map(|x| x.column).unique().collect();

        for row in self.rows.iter_mut() {
            if !occupied_rows.contains(&row.original_z) {
               row_exp += distance;  
            } 
            
            row.expanded_z = row.expanded_z + row_exp; 

         } 
        for col in self.cols.iter_mut() {
            if !occupied_cols.contains(&col.original_z) {
                col_exp += distance; 
            }
            col.expanded_z = col.expanded_z + col_exp;
        }
    } 

    fn get_expanded_coords(&self, g: &Galaxy) -> (usize, usize) {
        let x = self.cols.get(g.column).unwrap().expanded_z;
        let y = self.rows.get(g.row   ).unwrap().expanded_z;
        
        (x,y)
    } 
}

fn part1(input: Universe) -> i64 {
    input.galaxies.iter().combinations_with_replacement(2).map(|gs| {
        let (vx, vy) = input.get_expanded_coords(gs[0]);
        let (ux, uy) = input.get_expanded_coords(gs[1]);
        
        // Calculate distance
        (ux as i64 - vx as i64).abs() + (uy as i64 - vy as i64).abs()

    }).sum()
} 

fn main() {
   let mut test_univers = Universe::new(TEST_1);
   test_univers.expand(1);
   assert_eq!(test_univers.galaxies.len(), 9);
   assert_eq!(test_univers.get_expanded_coords(test_univers.galaxies.get(5).unwrap()), (12,7));
   let test_part1 = part1(test_univers);
   println!("[PART 1 TEST]:  {}", test_part1 );
   assert_eq!(test_part1, 374);

   let mut input_universe = Universe::new(INPUT); 
   input_universe.expand(1);
    
   let part1_out = part1(input_universe);
   println!("[PART 1 OUTPUT]:  {}", part1_out );
    
   let mut test2_uni  = Universe::new(TEST_1);
   test2_uni.expand(99);
   assert_eq!(part1(test2_uni), 8410);

   let mut part2_universe = Universe::new(INPUT); 
   part2_universe.expand(999999);
   println!("[PART 2 OUTPUT]: {} ", part1(part2_universe))


} 

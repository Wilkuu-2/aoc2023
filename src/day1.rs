
// 1abc2 -> 12 

fn part1(_input: &str) -> i32 {
    let mut outputs: Vec<i32> = Vec::new();
    for l in _input.lines(){ 
        let mut num1: i32 = -1; 
        let mut num2: i32 = -1; 
        for c in l.chars(){
            if let Some(digit) = c.to_digit(10)
            {
               if num1 < 0{
                   num1 = digit.try_into().unwrap(); 
               } 
               else {
                   num2 = digit.try_into().unwrap(); 
                } 
            }
            if num2 < 0 {
                num2 = num1;
            }
        }
        outputs.push(num2 + 10 * num1); 
    }
    outputs.iter().sum() 
} 


const DIGIT_SPELLED: [&str; 9] = ["one","two","three","four","five","six","seven","eight","nine"]; 

fn part2(_input: &str) -> i32 {
    let mut outputs: Vec<i32> = Vec::new();
    for line in _input.lines(){
       let mut chars = line.chars().into_iter(); 
       let mut num1: i32 = -1;
       let mut num2: i32 = -1; 
       let mut word: String = String::new(); 
       while let Some(c) = chars.next(){
            let mut digit: i32 = -1; 
            if let Some(d) = c.to_digit(10) {
                digit = d.try_into().unwrap(); // Can't possibly be bigger than 2^16;
                word.truncate(0); 
            } else {
                word.push(c); 
                if word.len() >= 3{
                    for i in 1..10{
                        if let Some(_) = word.strip_suffix(DIGIT_SPELLED[i-1]) {
                            // word.truncate(0);
                            digit = i.try_into().unwrap(); // i is in between 1 and 9 so it can't overflow 
                            break; 
                        }
                    } 
                }
            }

            if digit > 0 {
                if num1 > 0 {
                    num2 = digit;
                } else {
                    num1 = digit;
                }
            }
            
        } 
        if num2 < 0 {
            num2 = num1;
        }
        //println!("{}, {}{}",line,num1,num2);
        outputs.push(num2 + 10 * num1); 
    }
    //println!("{:?}", outputs);
    outputs.iter().sum()
} 

const TEST_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

const TEST_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
fiveight"#;

const INPUT: &str = include_str!("../inputs/day1.txt");

fn main() {
    println!("[------------ PART 1 ------------]");
    // println!("[TEST]\n{}", TEST_1);
    println!("[TEST OUTPUT]: {:?}", part1(TEST_1));
    println!("[OUTPUT]: {:?}", part1(INPUT));
    println!("[------------ PART 2 ------------]");
    println!("[TEST OUTPUT]: {:?}", part2(TEST_2));
    println!("[OUTPUT]: {:?}", part2(INPUT));

}


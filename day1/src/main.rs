use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let lines = read_lines("./src/input");
    
    let re = Regex::new(r"([LR])(\d+)").unwrap();

    let mut position = 50;
    let mut out = 0;
    for line in lines.map_while(Result::ok) {
        let mat = re.captures(&(line.as_str())).unwrap();
        let lr = mat[1].eq("L");
        let amt: i32 = mat[2].parse().unwrap();
        
        let change = amt * {if lr {-1} else {1}};
        let scorechange = if lr {
            (position - if position > 0 {100} else {0} - amt) / -100
        } else {
            (position + amt) / 100
        };
        out += scorechange;

        position += change;

        position %= 100;
        if position < 0 {position += 100};

        // part 1 code
        // if position == 0 {
        //     out += 1;
        // }
    }
    println!("{out}");
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
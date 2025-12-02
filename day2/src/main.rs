use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let lines = read_lines("./src/input");
    
    let re = Regex::new(r"(\d+)-(\d+),?").unwrap();
    
    let mut out = 0;
    // I'm sure there's a clever and efficient algorithmic method
    // However, I don't feel like being particularly clever rn, so HashSet is fast enough
    let mut set = HashSet::new();
    for line in lines.map_while(Result::ok) {
        for (_, [start, end]) in re.captures_iter(&line).map(|c| c.extract()){
            let start : u64 = start.parse().expect("Could not parse starting value.");
            let end : u64 = end.parse().expect("Could not parse ending value.");

            let start_pow = start.ilog10();
            let end_pow = end.ilog10();
            let end_halfpow = (end_pow + 1) / 2;
            
            // println!("Start pow: {start_pow}");
             
            for i in 1..=end_halfpow {
                // println!("i: {i}");
                for j in start_pow..=end_pow {
                    if (j + 1) % i == 0 {
                        let mut upper = 10u64.pow(i - 1);
                        let mut rep = repeat(upper, (j + 1) / i);
                        while rep <= end {
                            if rep >= start {
                                set.insert(rep);
                                // println!("src: {start}-{end}, upper: {upper} -> rep'd: {rep}");
                            }
                            upper += 1;
                            rep = repeat(upper, (j + 1) / i);
                        }
                    }
                }
            }
        }
    }

    for i in set {
        out += i;
    }
    println!("{out}");
}

fn repeat(upper: u64, times: u32) -> u64 {
    let exp = upper.ilog10() + 1;
    if times == 1 { return u64::MAX; }
    let mut res = 0;
    for _ in 0..times {
        res = res * 10u64.pow(exp) + upper;
    }
    res
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
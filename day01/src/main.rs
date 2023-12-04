use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(142, part_a("sample_a.txt"));
    assert_eq!(281, part_b("sample_b.txt"));

    println!("Part A: {}", part_a("input.txt"));
    println!("Part B: {}", part_b("input.txt"));

    Ok(())
}

fn part_a(file: &str) -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([
        ("1", 1), ("2", 2), ("3",3), ("4", 4), ("5", 5), 
        ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ]);
    solve(file, map)
}

fn part_b(file: &str) -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([
        ("1", 1), ("2", 2), ("3",3), ("4", 4), ("5", 5), ("6", 6), ("7", 7),
        ("8", 8), ("9", 9), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ]);
    solve(file, map)
}

fn solve(file: &str, map: HashMap<&str, i32>) -> i32 {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap())
        .map(|st| {
            let mut first_number: Option<&i32> = None;
            let mut last_number: Option<&i32> = None;

            let s = st.as_str();
            let l = s.len();
            for i in 0..l {
                for j in 0..i {
                    if first_number.is_none() && map.contains_key(&s[j..i]) {
                        first_number = map.get(&s[j..i]);
                    }
                    if last_number.is_none() && map.contains_key(&s[l-i..l-j]) {
                        last_number = map.get(&s[l-i..l-j]);
                    }
                }
                
                if first_number.is_none() && map.contains_key(&s[i..i+1]) {
                    first_number = map.get(&s[i..i+1]);
                }

                if last_number.is_none() && map.contains_key(&s[l-i-1..l-i]) {
                    last_number = map.get(&s[l-i-1..l-i]);
                }   

                if !first_number.is_none() && !last_number.is_none() {
                    break;
                }
            }
            if let (Some(first), Some(last)) = (first_number, last_number) {
                first * 10 + last
            } else {
                -1
            }
        })
        .sum::<i32>()
}

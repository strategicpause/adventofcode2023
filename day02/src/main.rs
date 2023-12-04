use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(8, part_a("sample.txt"));
    assert_eq!(2286, part_b("sample.txt"));

    println!("Part A: {}", part_a("input.txt"));
    println!("Part B: {}", part_b("input.txt"));

    Ok(())
}

fn part_a(file: &str) -> i32 {
    let map: HashMap<&str, i32> = HashMap::from([
        ("red", 12), ("green", 13), ("blue", 14)
    ]);
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap())
        .map(|st| {
            let game: Vec<&str> = st.split(':').collect();
            if is_game_not_possible(game[1], &map) {
                0
            } else {
                match game[0][5..].parse::<i32>() {
                    Ok(game_num) => game_num,
                    Err(_) => 0,
                }
            }
        })
        .sum::<i32>()
}

fn is_game_not_possible(game: &str, map: &HashMap<&str, i32>) -> bool {
    game.split(';')
        .map(|s| s.trim())
        .flat_map(|s| s.split(','))
        .map(|s| s.trim())
        .any(|grab| is_grab_not_possible(grab, map))
}

fn is_grab_not_possible(grab: &str, map: &HashMap<&str, i32>) -> bool {
    let result: Vec<&str> = grab.split(' ').collect();
    let (amount, color) = (result[0], result[1]);
    match map.get(color) {
        Some(max_cubes) => {
            match amount.parse::<i32>() {
                Ok(num_cubes) => num_cubes > *max_cubes,
                Err(_) => true
            }
        }
        None => true
    }
}


fn part_b(file: &str) -> i32 {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap())
        .map(|st| {
            let mut map: HashMap<&str, i32> = HashMap::new();
            let game: Vec<&str> = st.split(':').collect();
            game[1].split(';')
                .flat_map(|s| s.split(','))
                .map(|s| s.trim())
                .map(|s| parse_grab(s))
                .for_each(|(color, num_cubes)| {
                    match map.get(color) {
                        Some(cubes) => {
                            if cubes < &num_cubes {
                                map.insert(color, num_cubes);
                            }
                        }
                        None => {
                            map.insert(color, num_cubes);
                        }
                    }
                });
            map.get("red").unwrap_or(&1) * map.get("blue").unwrap_or(&1) * map.get("green").unwrap_or(&1)
        })
        .sum::<i32>()
}

fn parse_grab(grab: &str) -> (&str, i32) {
    let result: Vec<&str> = grab.split(' ').collect();
    let (amount, color) = (result[0], result[1]);

    match amount.parse::<i32>() {
        Ok(num_cubes) => (color, num_cubes),
        Err(_) => (color, 1)
    }
}
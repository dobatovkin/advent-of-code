use std::{env, fs::File, io::{self, BufRead}, path::Path, process};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Specify first argument as file input");
        process::exit(1);
    }
    
    let lines = read_lines(args[1].as_str()).unwrap();
    
    let mut dial_pos : i64 = 50;
    let mut absolute_pos: i64 = dial_pos;
    
    let mut ans: u64 = 0;
    let mut ans_v2: u64 = 0;
    
    for line in lines {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.len() < 2 {
            eprintln!("Line '{}' is missing rotation value", trimmed);
            process::exit(1);
        }

        let (direction_str, rotation_str) = trimmed.split_at(1);
        let direction = direction_str.chars().next().unwrap();
        let rotation: i64 = rotation_str.parse().expect("Invalid rotation value");
        
        let delta = match direction {
            'R' => rotation,
            'L' => -rotation,
            _ => {
                eprintln!("Unexpected direction '{}'", direction);
                process::exit(1);
            }
        };

        let start_abs = absolute_pos;
        absolute_pos += delta;

        ans_v2 += count_zero_hits(start_abs, delta);

        dial_pos = normalize_dial_position(absolute_pos);

        if dial_pos == 0 {
            ans += 1;
        }
    }
    
    println!("Answer for the part 1 is {}", ans);
    println!("Answer for the part 2 is {}", ans_v2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn normalize_dial_position(pos: i64) -> i64 {
    const RANGE: i64 = 100;
    pos.rem_euclid(RANGE)
}

fn count_zero_hits(start: i64, delta: i64) -> u64 {
    if delta == 0 {
        return 0;
    }

    let end = start + delta;

    if delta > 0 {
        (end.div_euclid(100) - start.div_euclid(100)) as u64
    } else {
        ((start - 1).div_euclid(100) - (end - 1).div_euclid(100)) as u64
    }
}
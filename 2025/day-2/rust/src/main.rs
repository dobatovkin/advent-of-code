use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Specify first argument as file input");
        process::exit(1);
    }

    let buf = fs::read_to_string(args[1].as_str()).unwrap();

    let ranges: Vec<[u64; 2]> = buf
        .trim()
        .split(',')
        .map(|range| {
            let mut splitted = range.split('-').into_iter();
            [
                splitted.next().unwrap().parse::<u64>().unwrap(),
                splitted.next().unwrap().parse::<u64>().unwrap(),
            ]
        })
        .collect();

    let mut ans: u64 = 0;

    // ans for first part
    for range in &ranges {
        let start = range[0];
        let end = range[1];
        for i in start..=end {
            let mut i_str = i.to_string();
            if i_str.len() % 2 != 0 {
                continue;
            }

            let i_right = i_str.split_off(i_str.len().div_euclid(2));

            if i_str == i_right {
                ans += i
            }
        }
    }

    println!("Answer for the part 1 is {}", ans);

    let mut ans_v2: u64 = 0;
    //ans for second part
    for range in &ranges {
        let start = range[0];
        let end = range[1];
        for id in start..=end {
            let id_str = id.to_string();
            for test_len in 1..=id_str.len() {
                if (id_str.len() % test_len != 0) || (id_str.len() / test_len < 2) {
                    continue;
                }
                // SAFETY: remainder is 0, as tested one item eairlier
                let mut is_invalid = true;
                for i in 0..((id_str.len().div_euclid(test_len))-1) {
                    let left_start = i * test_len; 
                    let left_end = left_start + test_len; 
                    let right_start = left_end; 
                    let right_end = right_start + test_len;
                    
                    if id_str[left_start..left_end] == id_str[right_start..right_end] {
                        continue;
                    } else {
                        is_invalid=false;
                        break;
                    }
                }
                if is_invalid {
                    ans_v2 += id;
                    break;
                }
            };

        }
    }

    println!("Answer for the part 2 is {}", ans_v2);
}

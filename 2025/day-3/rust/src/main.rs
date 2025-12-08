use std::{
    env,
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Specify first argument as file input");
        process::exit(1);
    }

    let batteries = read_lines(args[1].as_str()).unwrap();
    let mut ans: u64 = 0;

    for bank_str in batteries {
        let bank_str = bank_str.unwrap();
        let bank_str = bank_str.trim();
        let bank: Vec<u8> = bank_str.bytes().map(|byte| byte - b'0').collect();
        let (max_battery_idx, max_battery) = bank
            .iter()
            .enumerate()
            .reduce(|acc, e| {
                // we should skip the last one as then the second battery will be empty
                if e.0 == bank.len() - 1 {
                    acc
                } else if acc.1 >= e.1 {
                    acc
                } else {
                    e
                }
            })
            .unwrap();

        let (_, bank_rem) = unsafe { bank.split_at_unchecked(max_battery_idx + 1) };

        let (_, max_second_battery) = bank_rem
            .iter()
            .enumerate()
            .reduce(|acc, e| if acc.1 > e.1 { acc } else { e })
            .unwrap();

        let max_joltage = max_battery * 10 + max_second_battery;
        println!("{}", max_joltage);
        ans += max_joltage as u64;
    }

    println!("Answer for the part 1 is {}", ans);

    let batteries = read_lines(args[1].as_str()).unwrap();
    let mut ans_v2: u64 = 0;
    const MAX_BATTERIES: usize = 12;

    for bank_str in batteries {
        let bank_str = bank_str.unwrap();
        let bank_str = bank_str.trim();
        let bank: Vec<u8> = bank_str.bytes().map(|byte| byte - b'0').collect();
        let mut start: usize = 0;
        let mut end: usize = bank.len() - MAX_BATTERIES;
        let mut chosen_batteries: Vec<u8> = Vec::with_capacity(MAX_BATTERIES);
        for _ in (0..MAX_BATTERIES) {
            let (max_battery_idx, max_battery) = bank[start..=end]
                .iter()
                .enumerate()
                .reduce(|acc, e| {
                    if acc.1 >= e.1 {
                        acc
                    } else {
                        e
                    }
                })
                .unwrap();
            start += max_battery_idx + 1;
            end+=1;
            chosen_batteries.push(*max_battery);
        }
        
        
        
        assert_eq!(chosen_batteries.len(), MAX_BATTERIES);
        let mut pow_ten = 0;
        while let Some(bat) = chosen_batteries.pop() {
            println!("{:?}, {}, {}, {}", chosen_batteries, pow_ten, 10u64.pow(pow_ten), bat as u64 * 10u64.pow(pow_ten));
            ans_v2+= bat as u64 * 10u64.pow(pow_ten);
            pow_ten+=1;
        }
        
    }

    println!("Answer for the part 2 is {}", ans_v2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

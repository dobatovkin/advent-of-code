use std::{
    collections::HashSet,
    env,
    fs::File,
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

    let lines_iter = read_lines(args[1].as_str()).unwrap();

    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();

    let mut ans: u64 = 0;

    let mut ingredients_start = false;
    for line in lines_iter {
        let line = line.unwrap();
        let line_trimmed = line.trim();

        if line_trimmed.is_empty() {
            ingredients_start = true;
            continue;
        } else if ingredients_start {
            let ingredient: u64 = line_trimmed.parse().unwrap();
            // as ingredients come right after ranges, we can check them right away
            match fresh_ranges
                .iter()
                .find(|&range| ingredient >= range.0 && ingredient <= range.1)
            {
                Some(_) => ans += 1,
                None => continue,
            }
        } else {
            let range: Vec<u64> = line_trimmed
                .split('-')
                .map(|string| string.parse::<u64>().unwrap())
                .collect();
            fresh_ranges.push((range[0], range[1]));
        }
    }

    println!("Answer for part 1 is {}", ans);

    // attempt 1: was too slow

    // let mut fresh_ids_set = HashSet::new();

    // for range in fresh_ranges {
    //     for id in range.0..range.1 {
    //         fresh_ids_set.insert(id);
    //     }
    // }

    // attempt 2: likely produced overlapping ranges
    // let mut merged: Vec<(u64, u64)> = Vec::new();
    // 'outer: for (start, end) in fresh_ranges {
    //     // check if it can be merged with any other range or pushed to merged vec
    //     for (mstart, mend) in merged.iter_mut() {
    //         if *mstart <= start && *mend >= end {
    //             // this is already covered range, can be omitted
    //             continue 'outer;
    //         } else if *mstart > start && *mend >= end {
    //             // this can be covered by extending the start of the range
    //             *mstart = start;
    //             continue 'outer;
    //         } else if *mstart <= start && *mend < end {
    //             // this can be covered by extending the end
    //             *mend = end;
    //             continue 'outer;
    //         } else if *mstart > start && *mend < end {
    //             // this can be covered by extending both start and the end
    //             *mend = end;
    //             *mstart = start;
    //             continue 'outer;
    //         }
    //         // by this point we have no intersections bw 2 ranges
    //     }
    //     // if no range from merged vec had intersections with this range, we need to add it to the list of ranges
    //     merged.push((start, end));
    // }

    let merged = merge_ranges(fresh_ranges);
    
    // Sum inclusive lengths
    let ans_v2: u64 = merged.iter().map(|(s, e)| e - s + 1).sum();
    println!("Answer for part 2 is {}", ans_v2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// attempt 3: finally worked
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }
    // sort by start, then end. sh
    ranges.sort_unstable_by(|a, b| {
        let ord = a.0.cmp(&b.0);
        if ord == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            ord
        }
    });

    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    let mut cur_start = ranges[0].0;
    let mut cur_end = ranges[0].1;

    for (start, end) in ranges.into_iter().skip(1) {
        if start <= cur_end {
            // overlaps or touches; extend
            if end > cur_end {
                cur_end = end;
            }
        } else {
            // disjoint, flush current and start new
            merged.push((cur_start, cur_end));
            cur_start = start;
            cur_end = end;
        }
    }
    merged.push((cur_start, cur_end));
    merged
}
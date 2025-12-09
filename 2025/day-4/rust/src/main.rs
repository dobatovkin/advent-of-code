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

    let matrix: Vec<Vec<bool>> = read_lines(args[1].as_str())
        .unwrap()
        .map(|row| row.unwrap().bytes().map(|byte| byte == b'@').collect())
        .collect();
    
    let mut ans: u64 = 0;
    
    // adjacent element names
    // tl tc tr -- top 
    // cl @  cr -- center
    // bl bc br -- bottom
    // |  |  |
    // |  |  right
    // |  center 
    // left

    for (row_idx, row) in matrix.iter().enumerate() {
        for (column_idx, e) in row.iter().enumerate() {
            if !e {
                continue;
            }
            let tl = matrix.get(row_idx.overflowing_sub(1).0).map(|top_row| top_row.get(column_idx.overflowing_sub(1).0).unwrap_or(&false)).unwrap_or(&false);
            let tc = matrix.get(row_idx.overflowing_sub(1).0).map(|top_row| top_row.get(column_idx).unwrap_or(&false)).unwrap_or(&false);
            let tr = matrix.get(row_idx.overflowing_sub(1).0).map(|top_row| top_row.get(column_idx+1).unwrap_or(&false)).unwrap_or(&false);
            let cl = matrix.get(row_idx).map(|top_row| top_row.get(column_idx.overflowing_sub(1).0).unwrap_or(&false)).unwrap_or(&false);
            let cr = matrix.get(row_idx).map(|top_row| top_row.get(column_idx+1).unwrap_or(&false)).unwrap_or(&false);
            let bl = matrix.get(row_idx+1).map(|top_row| top_row.get(column_idx.overflowing_sub(1).0).unwrap_or(&false)).unwrap_or(&false);
            let bc = matrix.get(row_idx+1).map(|top_row| top_row.get(column_idx).unwrap_or(&false)).unwrap_or(&false);
            let br = matrix.get(row_idx+1).map(|top_row| top_row.get(column_idx+1).unwrap_or(&false)).unwrap_or(&false);
            
            let mut count: u8 = 0;
            [tl, tc, tr, cl, cr, bl, bc, br].iter().for_each(|e| if **e {count+=1});
            if count < 4 {
                ans += 1;
            }
        }
    }
    
    println!("Answer for the part 1 is {}", ans);
    
    let mut matrix: Vec<Vec<bool>> = read_lines(args[1].as_str())
        .unwrap()
        .map(|row| row.unwrap().bytes().map(|byte| byte == b'@').collect())
        .collect();
    
    let mut ans_v2: u64 = 0;
    
    loop {
        let mut forklift_queue = Vec::new();
        for (row_idx, row) in matrix.iter().enumerate() {
            for (column_idx, e) in row.iter().enumerate() {
                if !e {
                    continue;
                }
                
                // SAFETY: overflowing for subtration is ok as we're not dealing with matrixes near usize::MAX in both dimentions 
                // ((2^64 − 1)^2 on 64-bit targets)
                let tl = matrix.get(row_idx.overflowing_sub(1).0).map(|top_row| top_row.get(column_idx.overflowing_sub(1).0).unwrap_or(&false)).unwrap_or(&false);
                let tc = matrix.get(row_idx.overflowing_sub(1).0).map(|top_row| top_row.get(column_idx).unwrap_or(&false)).unwrap_or(&false);
                let tr = matrix.get(row_idx.overflowing_sub(1).0).map(|top_row| top_row.get(column_idx+1).unwrap_or(&false)).unwrap_or(&false);
                let cl = matrix.get(row_idx).map(|top_row| top_row.get(column_idx.overflowing_sub(1).0).unwrap_or(&false)).unwrap_or(&false);
                let cr = matrix.get(row_idx).map(|top_row| top_row.get(column_idx+1).unwrap_or(&false)).unwrap_or(&false);
                let bl = matrix.get(row_idx+1).map(|top_row| top_row.get(column_idx.overflowing_sub(1).0).unwrap_or(&false)).unwrap_or(&false);
                let bc = matrix.get(row_idx+1).map(|top_row| top_row.get(column_idx).unwrap_or(&false)).unwrap_or(&false);
                let br = matrix.get(row_idx+1).map(|top_row| top_row.get(column_idx+1).unwrap_or(&false)).unwrap_or(&false);
                
                let mut count: u8 = 0;
                [tl, tc, tr, cl, cr, bl, bc, br].iter().for_each(|e| if **e {count+=1});
                if count < 4 {
                    forklift_queue.push((row_idx, column_idx));
                }
            }
        }
        
        if forklift_queue.len() == 0 {
            break;
        }
        
        ans_v2 += forklift_queue.len() as u64;
        
        for (row, col) in forklift_queue {
            matrix[row][col] = false;
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

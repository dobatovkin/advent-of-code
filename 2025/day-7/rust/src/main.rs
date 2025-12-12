use std::{
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

    let mut matrix: Vec<Vec<u8>> = lines_iter
        .map(|line| line.unwrap().as_bytes().into_iter().cloned().collect())
        .collect();

    let mut split_count: u64 = 0;

    //attempt 1: it was a mistake to lookup the prev and another previous line - it is better to look into the current and the next one?
    // but funny enough - it produces correct value :)
    //
    // for i in 1..matrix.len() {
    //     for j in 0..matrix[i].len() {
    //         match matrix[i - 1][j] {
    //             b'S' => {
    //                 matrix[i][j] = fill_ray_byte(matrix[i][j]);
    //             }
    //             b'^' => {
    //                 if matrix[i - 2][j] == b'|' {
    //                     matrix[i][j + 1] = fill_ray_byte(matrix[i][j + 1]);
    //                     matrix[i][j - 1] = fill_ray_byte(matrix[i][j - 1]);
    //                     split_count += 1;
    //                 }
    //             }
    //             b'|' => {
    //                 matrix[i][j] = fill_ray_byte(matrix[i][j]);
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                b'S' => {
                    if i != matrix.len() - 1 {
                        matrix[i + 1][j] = fill_ray_byte(matrix[i + 1][j]);
                    }
                }
                b'^' => {
                    // SAFETY: first line does not contain splitters
                    if matrix[i - 1][j] == b'|' {
                        matrix[i][j + 1] = fill_ray_byte(matrix[i][j + 1]);
                        matrix[i][j - 1] = fill_ray_byte(matrix[i][j - 1]);
                        if i != matrix.len() - 1 {
                            matrix[i + 1][j - 1] = fill_ray_byte(matrix[i + 1][j - 1]);
                        };
                        split_count += 1;
                    }
                }
                b'|' => {
                    if i != matrix.len() - 1 {
                        matrix[i + 1][j] = fill_ray_byte(matrix[i + 1][j]);
                    }
                }
                _ => {}
            }
        }
    }

    // let display_matrix: Vec<String> = matrix
    //     .iter().cloned()
    //     .map(|l| String::from_utf8(l).unwrap())
    //     .collect();
    // println!("{:#?}", display_matrix);

    println!("Answer for part 1 is {}", split_count);

    let zero_line = vec![0u64; matrix[0].len()];
    let mut acc_matrix: Vec<Vec<u64>> = vec![zero_line; matrix.len()];

    for i in 0..matrix[0].len() {
        match matrix.last().unwrap()[i] {
            b'|' => {
                acc_matrix[matrix[0].len() - 1][i] = 1u64;
            }
            _ => {}
        }
    }

    for i in (0..(matrix.len() - 2)).rev() {
        for j in 0..matrix[0].len() {
            match matrix[i][j] {
                b'|' | b'S' => {
                    acc_matrix[i][j] = match matrix[i + 1][j] {
                        b'|' => acc_matrix[i + 1][j],
                        b'^' => acc_matrix[i + 1][j - 1] + acc_matrix[i + 1][j + 1],
                        _ => 0,
                    }
                }
                _ => {}
            }
        }
    }

    // let display_mtx: Vec<String> = acc_matrix
    //     .iter().cloned()
    //     .map(|vec| {
    //         vec.iter()
    //             .map(|num| num.to_string())
    //             .collect::<Vec<String>>()
    //             .join(" ")
    //     })
    //     .collect();
    // println!("{:#?}", display_mtx);

    println!(
        "Answer for part 2 is {}",
        acc_matrix[0].iter().max().unwrap()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fill_ray_byte(b: u8) -> u8 {
    match b {
        b'.' => b'|',
        _ => b,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn timelines_test() {
        let input = r#".......S.......
                .......|.......
                ......|^|......
                ......|.|......
                .....|^|^|.....
                .....|.|.|.....
                ....|^|^|^|....
                ....|.|.|.|....
                ...|^|^|||^|...
                ...|.|.|||.|...
                ..|^|^|||^|^|..
                ..|.|.|||.|.|..
                .|^|||^||.||^|.
                .|.|||.||.||.|.
                |^|^|^|^|^|||^|
                |.|.|.|.|.|||.|"#;

        let matrix: Vec<Vec<u8>> = input
            .trim()
            .split("\n")
            .map(|line| line.trim().as_bytes().into_iter().cloned().collect())
            .collect();

        let zero_line = vec![0u64; matrix[0].len()];
        let mut acc_matrix: Vec<Vec<u64>> = vec![zero_line; matrix.len()];

        // fill first row

        for i in 0..matrix[0].len() {
            match matrix.last().unwrap()[i] {
                b'|' => {
                    acc_matrix[matrix[0].len() - 1][i] = 1u64;
                }
                _ => {}
            }
        }

        for i in (0..(matrix.len() - 2)).rev() {
            for j in 0..matrix[0].len() {
                println!("{i}, {j}");
                match matrix[i][j] {
                    b'|' | b'S' => {
                        acc_matrix[i][j] = match matrix[i + 1][j] {
                            b'|' => acc_matrix[i + 1][j],
                            b'^' => acc_matrix[i + 1][j - 1] + acc_matrix[i + 1][j + 1],
                            _ => 0,
                        }
                    }
                    _ => {}
                }
            }
        }

        let display_mtx: Vec<String> = acc_matrix
            .iter()
            .cloned()
            .map(|vec| {
                vec.iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect();
        println!("{:#?}", display_mtx);

        assert_eq!(40, *acc_matrix[0].iter().max().unwrap())
    }
}

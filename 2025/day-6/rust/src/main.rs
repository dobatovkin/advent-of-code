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

    let mut cephalopod_matrix: Vec<Vec<String>> = Vec::new();

    for line in lines_iter {
        let line = line.unwrap();
        cephalopod_matrix.push(
            line.split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
    }

    let operators: Vec<String> = cephalopod_matrix.pop().unwrap();

    let operands: Vec<Vec<u64>> = cephalopod_matrix
        .iter()
        .map(|line| line.iter().map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut results: Vec<u64> = Vec::with_capacity(operators.len());

    for i in 0..operators.len() {
        let operands: Vec<u64> = operands.iter().map(|v| v[i]).collect();
        let result: u64 = match operators[i].as_str() {
            "*" => operands.iter().cloned().reduce(|acc, e| acc * e).unwrap(),
            "+" => operands.iter().cloned().reduce(|acc, e| acc + e).unwrap(),
            _ => 0,
        };
        results.push(result);
    }

    let ans: u64 = results.iter().sum();
    println!("Answer for part 1 is {:?}", ans);

    let lines_iter = read_lines(args[1].as_str()).unwrap();

    let mut matrix_str: Vec<String> = lines_iter.map(|s| s.unwrap()).collect();
    let mut operands: Vec<Vec<u64>> = Vec::new();
    let operators: Vec<String> = matrix_str
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .rev()
        .collect();

    let matrix: Vec<Vec<u8>> = matrix_str
        .into_iter()
        .map(|e| e.as_bytes().to_vec())
        .collect();

    let mut operands_group = Vec::new();
    for i in 0..matrix[0].len() {
        // right to left
        let i = matrix[0].len() - (1 + i);

        let mut operand = String::new();
        for j in 0..matrix.len() {
            // top to bottom
            let char = matrix[j][i] as char;
            operand.push(char);
        }
        if operand.trim().len() == 0 {
            operands.push(operands_group);
            operands_group=Vec::new()
        } else {
            operands_group.push(operand.trim().parse().unwrap());
        }
    }
    operands.push(operands_group);
    let mut results: Vec<u64> = Vec::with_capacity(operators.len());


    for i in 0..operators.len() {
        let result: u64 = match operators[i].as_str() {
            "*" => operands[i].iter().cloned().reduce(|acc, e| acc * e).unwrap(),
            "+" => operands[i].iter().cloned().reduce(|acc, e| acc + e).unwrap(),
            _ => 0,
        };
        results.push(result);
    }

    let ans: u64 = results.iter().sum();
    
    println!("Answer for part 2 is {:?}", ans);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

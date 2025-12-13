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

    let mut junction_boxes: Vec<Coord> = Vec::with_capacity(1000);

    for line in lines_iter {
        let line = line.unwrap();
        junction_boxes.push(Coord::from_string(&line));
    }

    // compute distances
    let mut distances: Vec<(i128, usize, usize)> = Vec::with_capacity(1000 * 1000);
    for i in 0..junction_boxes.len() {
        // starting from i, as pairs before were already calculated
        for j in i..junction_boxes.len() {
            if i == j {
                continue;
            }
            let distance = junction_boxes[i].dist_sq_to_coord(&junction_boxes[j]);
            distances.push((distance, i, j));
        }
    }

    //sort distances
    distances.sort_unstable_by(|d1, d2| d1.0.cmp(&d2.0));

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for distance in distances.iter().take(1000) {
        for circuit in circuits.iter_mut() {
            if circuit.contains(&distance.1) || circuit.contains(&distance.2) {
                circuit.insert(distance.1);
                circuit.insert(distance.2);
                break;
            }
        }
        circuits.push(HashSet::from([distance.1, distance.2]));
    }

    // merge circuits
    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i < circuits.len() {
            let mut j = i + 1;
            while j < circuits.len() {
                if !circuits[i].is_disjoint(&circuits[j]) {
                    // merge j into i
                    let other_circuit = circuits.remove(j);
                    circuits[i].extend(other_circuit);
                    changed = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    let mut circuits_sizes = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();

    circuits_sizes.sort_unstable();

    let largest_circuits_sizes = circuits_sizes.last_chunk::<3>().unwrap();

    let ans: usize = largest_circuits_sizes.iter().product();
    println!("Answer for part 1 is {}", ans);

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for i in 0..junction_boxes.len() {
        circuits.push(HashSet::from([i]));
    }

    for distance in distances.iter() {
        for circuit in circuits.iter_mut() {
            if circuit.contains(&distance.1) || circuit.contains(&distance.2) {
                circuit.insert(distance.1);
                circuit.insert(distance.2);
                break;
            }
        }
        // merge circuits - not the most optimal solution - merging them in the previous loop makes more sense
        let mut changed = true;
        while changed {
            changed = false;
            let mut i = 0;
            while i < circuits.len() {
                let mut j = i + 1;
                while j < circuits.len() {
                    if !circuits[i].is_disjoint(&circuits[j]) {
                        // merge j into i
                        let other_circuit = circuits.remove(j);
                        circuits[i].extend(other_circuit);
                        changed = true;
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
        }
        if circuits.len() == 1 {
            let ans =
                (junction_boxes[distance.1].x as i128) * (junction_boxes[distance.2].x as i128);
            println!("Answer for part 2 is {}", ans);
            break;
        }
    }
}

struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn from_string(coord_string: &str) -> Self {
        let mut coord_iter = coord_string.split(',');
        Self {
            x: coord_iter.next().unwrap().parse().unwrap(),
            y: coord_iter.next().unwrap().parse().unwrap(),
            z: coord_iter.next().unwrap().parse().unwrap(),
        }
    }
    // float distance caused rounding errors it seems - changing that fixed the issue
    fn dist_sq_to_coord(&self, next_coord: &Coord) -> i128 {
        let dx = (self.x - next_coord.x) as i128;
        let dy = (self.y - next_coord.y) as i128;
        let dz = (self.z - next_coord.z) as i128;
        dx * dx + dy * dy + dz * dz
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

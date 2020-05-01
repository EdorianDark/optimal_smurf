use optimal_smurf::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).unwrap();

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lines = contents.lines();
    let first_line = lines.next().expect("file empty");
    let first_line: Vec<i64> = first_line
        .split(" ")
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect();
    let num_elements = first_line[0] as usize;
    let capacity = first_line[1];
    let mut v = vec![];
    let mut w = vec![];
    for line in lines {
        let line: Vec<i64> = line
            .split(" ")
            .map(|s| s.parse::<i64>())
            .filter_map(Result::ok)
            .collect();
        if line.is_empty() {
            continue;
        }
        v.push(line[0]);
        w.push(line[1]);
    }
    assert_eq!(num_elements, v.len());

    let problem = build_problem(v, w, capacity);
    let solved = bounding_solve(problem);

    let mut out: String = format!("{0} 0\n", solved.value);
    for element in solved.contained {
        if element {
            out.push_str("1 ");
        } else {
            out.push_str("0 ");
        }
    }

    print!("{}", out);

    return Ok(());
}

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
    let first_line: Vec<_> = first_line
        .split(" ")
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect();
    let num_nodes = first_line[0];
    let num_edges = first_line[1];
    let mut graph = Graph::new(num_nodes);

    for line in lines {
        let line: Vec<_> = line
            .split(" ")
            .map(|s| s.parse::<usize>())
            .filter_map(Result::ok)
            .collect();
        if line.is_empty() {
            continue;
        }
        graph.add_edge(line[0], line[1]);
    }
    
    let colord = color(&graph);
    let num_colors = colord.iter().max().unwrap();


    let mut out: String = format!("{0} 0\n", num_colors+1);
    for element in colord {
        out.push_str(&format!("{} ", element));
    }

    print!("{}", out);

    return Ok(());
}

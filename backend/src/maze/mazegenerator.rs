use petgraph::graph::UnGraph;
use petgraph::dot::{Dot, Config};
use std::io::{stdout, Write};
use std::fs::File;

pub fn maze_generator() {
    println!("THIS IS THE MAZE GENERATOR");
    let mut g = UnGraph::<i32, ()>::new_undirected();
    for i in 0..16 {
        g.add_node(i);
    }
    
    // Output the tree to `graphviz` `DOT` format
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}


pub fn visualize_maze() -> std::io::Result<()> {
    let maze = vec![
        vec!['#', '#', '#', '#', '#'],
        vec!['#', '.', '.', '.', '#'],
        vec!['#', '.', '#', '.', '#'],
        vec!['#', '.', '.', '.', '#'],
        vec!['#', '#', '#', '#', '#'],
    ];

    let mut file = File::create("maze.txt").expect("Failed to create file");
    
    Ok(for row in &maze {
        let line: String = row.iter().collect();
        writeln!(file, "{}", line).expect("Failed to write to file");
    })
}
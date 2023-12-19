pub mod node;

use std::fs::File;
use std::io::{BufRead, BufReader};

use node::*;



/**
 * Read the `paths.txt` file line by line and add each line to a vector.
 * @param path The path to the file to read.
 * @return A vector containing the lines of the file.
 */
fn read_file(path: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
     lines.push(line.expect("Could not read line"));
    }
    lines
}

fn main() {
    let lines = read_file("paths.txt");



    let mut tree = Tree::new();
    for line in lines {
        println!("");
        println!("[---------------------------------------------------");
        println!("Adding : {}", line);

        let default_path = line.clone();
        let mut split: Vec<&str> = line.split("/").collect();

        // Remove the first element if it is a dot.
        if split[0] == "." {
            split.remove(0);
        }

        // Remove the last element if it is empty.
        if split[split.len() - 1] == "" {
            split.remove(split.len() - 1);
        }

        let data = NodeData {
            length: default_path.len() as u64,
            default_path,
        };

        let depth = split.len() as u64;

        println!("Split: {:?}", split);


        let name = split[(depth - 1) as usize].to_string();
        let path = split.join("/");

        println!("Adding file: {}", name);
        println!("Path: {}", path);


        let node = if split[(depth - 1) as usize].contains(".") {
            Node::new_file(data, depth, path, name)
        } else {
            Node::new_directory(data, depth, path, name)
        };

        tree.insert(node);

        println!("");
        println!("---------------------------------------------------]");
    }


    tree.display();

    let mainrs = tree.find_by_name("main.rs".to_string());
    match mainrs {
        Some(node) => {
            node.display();
        },
        None => {
            println!("Could not find main.rs");
        }
    }
}

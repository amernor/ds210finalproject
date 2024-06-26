use std::fs::File;
use std::io::prelude::*;


//reads the vertices from the txt file into a vector 
pub fn read_file(path: &str) ->  Vec<(usize,usize)> {
    let mut result: Vec<(usize,usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader{
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x: usize = v[0].parse::<usize>().unwrap();
        let y: usize = v[1].parse::<usize>().unwrap();
        result.push((x,y));
    }
    return result;
}
 
use rand::seq::IteratorRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rng = rand::thread_rng();
    let faces = "😀😎😐😕😠😢";
    println!("I am {}!", faces.chars().choose(&mut rng).unwrap());

    let names = names_array_from_file("data/oreo_names.txt");
    for n in &names {
        println!("{n}");
    }
}

fn names_array_from_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("could not open file!");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

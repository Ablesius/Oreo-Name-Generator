use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use rand::seq::SliceRandom; // 0.7.2


fn main() {
    // 1. file contents to vec
    let names = names_array_from_file("data/oreo_names.txt");

    // 2. random choice(s) from vec
    let random_amount = rand::thread_rng().gen_range(1..=5);
    let selection = names
        .choose_multiple(&mut rand::thread_rng(), random_amount as usize)
        .collect::<Vec<_>>();

    // 4. print result + "Oreo"
    println!("{:?} Oreos", selection);
}

fn names_array_from_file(filepath: &str) -> Vec<String> {
    // TODO: bake file into binary with include_str!("filename");
    let file = File::open(filepath).expect("could not open file!");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        // there's almost no chance that a line is unreadable, so unwrap should be ok
        .map(|line| line.unwrap())
        .collect()
}

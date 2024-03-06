use rand::{seq::IteratorRandom, Rng};

fn main() {
    const NAMES: &str = include_str!("data/oreo_names.txt");

    let amount: u8 = rand::thread_rng().gen_range(1..=5);
    let selection = NAMES
        .lines()
        .choose_multiple(&mut rand::thread_rng(), amount as usize)
        .join(" ");

    println!("{} Oreos", selection);
}

use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    // 1. file contents to vec
    let names_str = include_str!("data/oreo_names.txt");
    let names: Vec<&str> = names_str.split('\n').collect();

    // 2. random choice(s) from vec
    let random_amount: u8 = rand::thread_rng().gen_range(1..=5);
    let selection = names
        .choose_multiple(&mut rand::thread_rng(), random_amount as usize)
        .collect::<Vec<_>>();

    // 4. print result + "Oreo"
    println!("{:?} Oreos", selection);
}

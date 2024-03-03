use rand::seq::IteratorRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let faces = "😀😎😐😕😠😢";
    println!("I am {}!", faces.chars().choose(&mut rng).unwrap());
}

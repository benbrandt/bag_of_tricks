use rand::SeedableRng;
use rand_pcg::Pcg64;

use bag_of_tricks::character::Character;

fn main() {
    let mut rng = Pcg64::from_entropy();
    println!("{}", Character::gen(&mut rng));
}

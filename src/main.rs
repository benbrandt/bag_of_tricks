use bag_of_tricks::character::Character;
use rand::SeedableRng;
use rand_pcg::Pcg64;

fn main() {
    let mut rng = Pcg64::from_entropy();
    println!("{:#?}", Character::new(&mut rng));
}

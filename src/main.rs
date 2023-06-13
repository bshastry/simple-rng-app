use clap::{Arg, App};
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let matches = App::new("Simple RNG")
        .version("0.1.0")
        .author("Bhargava Shastry")
        .about("Generates random numbers")
        .arg(Arg::with_name("seed")
                 .short("s")
                 .long("seed")
                 .takes_value(true)
                 .help("Seed value for RNG"))
        .get_matches();

    let seed = matches.value_of("seed").unwrap_or("None");
    println!("The seed passed is: {}", seed);
    let seed_num: u64 = match seed.parse() {
        Ok(n) => { println!("The seed is {}.", n); n }
        Err(_) => { println!("That's not a number! {}", seed); 0 }
    };
    let mut rng = StdRng::seed_from_u64(seed_num);
    let random_number = rng.gen::<u32>();
    println!("The random number is: {}", random_number);
}


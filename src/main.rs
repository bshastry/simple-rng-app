use clap::ArgMatches;
use clap::{App, Arg};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn parser(matches: &ArgMatches) -> u64 {
    let seed = matches.value_of("seed").unwrap_or("None");
    println!("The seed passed is: {}", seed);
    let seed_num: u64 = match seed.parse() {
        Ok(n) => {
            println!("The seed is {}.", n);
            n
        }
        Err(_) => {
            println!("That's not a number! {}", seed);
            0
        }
    };
    let mut rng = StdRng::seed_from_u64(seed_num);
    let random_number = rng.gen::<u64>();
    println!("The random number is: {}", random_number);
    random_number
}

fn main() {
    parser(
        &App::new("Simple RNG")
            .version("0.1.0")
            .author("Bhargava Shastry")
            .about("Generates random numbers")
            .arg(
                Arg::with_name("seed")
                    .short("s")
                    .long("seed")
                    .takes_value(true)
                    .help("Seed value for RNG"),
            )
            .get_matches(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_seed() {
        let seed = "1234";
        let matches = App::new("Simple RNG")
            .arg(
                Arg::with_name("seed")
                    .short("s")
                    .long("seed")
                    .takes_value(true)
                    .help("Seed value for RNG"),
            )
            .get_matches_from(vec!["simplerng", "--seed", seed]);
        assert_eq!(2185187624241326233, parser(&matches));
    }

    #[test]
    fn test_without_seed() {
        let matches = App::new("Simple RNG").get_matches_from(vec!["simplerng"]);
        assert_eq!(13486662071293341567, parser(&matches));
    }
}

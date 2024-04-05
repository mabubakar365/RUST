use std::fmt;

/* instructs the Rust compiler to automatically generate an implementation of the Debug trait for Footballer struct */
#[derive(Debug)]
struct Footballer {
    /* Data Type: String */
    name: String,
    /* Data Type: String */
    country: String,
    /* Data Type: Unsigned Integer 32 bit */
    goals_scored: u32,
    /* Data Type: Unsigned Integer 32 bit */
    matches_played: u32,
    /* Data Type: flexible list */
    trophies_won: Vec<String>, 
}

/* whenever Rust sees the {} placeholder, it calls the Display trait for the type being printed. It gives the trait implementation two pointers: one to the formatter buffer (where the output goes) and another to the structure itself */
impl fmt::Display for Footballer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /* write the formatted output to the buffer using the write! macro */
        write!(f, "{} ({}) - Goals: {}, Matches: {}, Trophies: {} ",
            self.name, self.country, self.goals_scored, self.matches_played, self.trophies_won.join(", "))
    }
}

fn main() {
    let player = Footballer{
        name: "Lionel Messi".to_string(),
        country: "Argentina".to_string(),
        goals_scored: 796,
        matches_played: 1073,
        trophies_won: vec!["Champion Leage".to_string(), "Laliga".to_string(), "Copa Del Ray".to_string(), "World Cup".to_string()]
    };

    /* With #[derive(Debug)], we can print a Footballer instance using the {:?} placeholder for Debug formatting. */
    println!("{:?}", player);

    /* With #[derive(Debug)], we can print a Footballer instance using the {:#?} placeholder for Debug formatting. */
    println!("{:#?}", player);

    println!("{}", player);
}

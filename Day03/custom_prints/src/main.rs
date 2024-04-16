/*
    Debug and Display Traits in RUST
    Printing custom data types like structures is crucial for debugging, analysis, and user communication.
    ({:?} for Debug and {} for Display)

    Debug provides a detailed, internal representation suitable for debugging, while Display allows for user-friendly output control.

    Debug Trait
     Rust automatically generates a Debug implementation using the #[derive(Debug)] attribute. 
     This trait focuses on providing a detailed, technical view of a data structure, making it ideal for debugging.
     The Debug trait is perfect for quick debugging sessions when you need to inspect the internal details of a structure or payload. 
     However, it might not be ideal for user-facing logs. 
     In production environments, you often want control over what information is displayed, and you might want to add prefixes or suffixes to make the messages easier for users to understand. 
     Rust provides a solution for this using the Display trait.

    Display Trait
     Unlike the Debug trait, the Display trait doesn't come automatically. Rust wants you to provide a custom implementation for the Display trait. 
     The main idea is that whenever Rust sees the {} placeholder, it calls the Display trait for the type being printed. 

*/
use std::fmt;

/*  #[derive(Debug)] instructs the Rust compiler to automatically generate an implementation of the Debug trait for Footballer struct 
    With #[derive(Debug)], we can now print a Footballer instance using the {:?} placeholder for Debug formatting.
*/
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

/* 
    impl fmt::Display for Footballer instructs to implement the Display trait for Footballer struct 
    whenever Rust sees the {} placeholder, it calls the Display trait for the type being printed. 
    It gives the trait implementation two pointers: one to the formatter buffer (f: &mut fmt::Formatter<'_> where the output goes) and another to the structure itself (&self).
    This gives the developer full control to format and represent the structure's data however they want, and then write that formatted output to the buffer using the write! macro.
*/
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

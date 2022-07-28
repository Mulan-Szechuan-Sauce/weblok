mod api;
mod names;
pub use api::*;

pub mod utils {
    use super::names;
    use rand::{self, prelude::SliceRandom};

    pub fn title_case(s: String) -> String {
        s.split_whitespace().map(|word| {
            let mut chars = word.chars();
            format!("{}{}", chars.next().unwrap().to_uppercase(), chars.collect::<String>())
        }).collect::<Vec<String>>().join(" ")
    }

    pub fn generate_username() -> String {
        let mut rng = rand::thread_rng();
        let adj  = names::ADJECTIVES.choose(&mut rng).unwrap();
        let poke = names::POKEMON.choose(&mut rng).unwrap();
        title_case(format!("{} {}", adj, poke))
    }
}

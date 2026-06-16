use serde::{Deserialize, Serialize};

use crate::models::pokemon_response::PokemonResponse;

pub const DESCRIPTION_NOT_AVAILABLE: &str = "No description availvable";

#[derive(Deserialize, Serialize)]
pub struct PokemonDTO {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

impl From<PokemonResponse> for PokemonDTO {
    fn from(r: PokemonResponse) -> Self {
        let mut english_flavor_texts = r
            .flavor_text_entries
            .iter()
            .filter(|flavor_text| flavor_text.language.name == "en");

        let description = english_flavor_texts
            .next()
            .map_or(DESCRIPTION_NOT_AVAILABLE.to_owned(), |e| {
                e.flavor_text.replace(['\n', '\x0c'], " ")
            });

        let habitat = match r.habitat {
            Some(h) => h.name,
            None => String::from("unknown"),
        };

        Self {
            name: r.name,
            description,
            habitat,
            is_legendary: r.is_legendary,
        }
    }
}

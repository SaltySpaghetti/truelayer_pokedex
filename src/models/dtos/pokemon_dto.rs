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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::pokemon_response::{EvolutionChain, FlavorTextEntry, Item, PokemonResponse};

    fn make_response(
        name: &str,
        is_legendary: bool,
        habitat: Option<&str>,
        flavor_texts: Vec<(&str, &str)>,
    ) -> PokemonResponse {
        PokemonResponse {
            base_happiness: 0,
            capture_rate: 45,
            color: Item {
                name: "blue".to_string(),
                url: String::new(),
            },
            egg_groups: vec![],
            evolution_chain: EvolutionChain { url: String::new() },
            evolves_from_species: None,
            flavor_text_entries: flavor_texts
                .into_iter()
                .map(|(lang, text)| FlavorTextEntry {
                    flavor_text: text.to_string(),
                    language: Item {
                        name: lang.to_string(),
                        url: String::new(),
                    },
                    version: Item {
                        name: "red".to_string(),
                        url: String::new(),
                    },
                })
                .collect(),
            form_descriptions: vec![],
            forms_switchable: false,
            gender_rate: -1,
            genera: vec![],
            generation: Item {
                name: "generation-i".to_string(),
                url: String::new(),
            },
            growth_rate: Item {
                name: "medium".to_string(),
                url: String::new(),
            },
            habitat: habitat.map(|h| Item {
                name: h.to_string(),
                url: String::new(),
            }),
            has_gender_differences: false,
            hatch_counter: 20,
            id: 1,
            is_baby: false,
            is_legendary,
            is_mythical: false,
            name: name.to_string(),
            names: vec![],
            order: 1,
            pal_park_encounters: vec![],
            pokedex_numbers: vec![],
            shape: Item {
                name: "upright".to_string(),
                url: String::new(),
            },
            varieties: vec![],
        }
    }

    #[test]
    fn test_from_uses_first_english_description() {
        let r = make_response("bulbasaur", false, None, vec![("en", "A seed pokemon.")]);
        assert_eq!(PokemonDTO::from(r).description, "A seed pokemon.");
    }

    #[test]
    fn test_from_no_english_description_uses_default() {
        let r = make_response("bulbasaur", false, None, vec![("fr", "Un pokemon.")]);
        assert_eq!(PokemonDTO::from(r).description, DESCRIPTION_NOT_AVAILABLE);
    }

    #[test]
    fn test_from_empty_flavor_texts_uses_default() {
        let r = make_response("bulbasaur", false, None, vec![]);
        assert_eq!(PokemonDTO::from(r).description, DESCRIPTION_NOT_AVAILABLE);
    }

    #[test]
    fn test_from_description_replaces_newlines() {
        let r = make_response("bulbasaur", false, None, vec![("en", "Line1\nLine2")]);
        assert_eq!(PokemonDTO::from(r).description, "Line1 Line2");
    }

    #[test]
    fn test_from_first_english_entry_wins_when_multiple() {
        let r = make_response(
            "bulbasaur",
            false,
            None,
            vec![("en", "First."), ("en", "Second.")],
        );
        assert_eq!(PokemonDTO::from(r).description, "First.");
    }

    #[test]
    fn test_from_no_habitat_maps_to_unknown() {
        let r = make_response("bulbasaur", false, None, vec![]);
        assert_eq!(PokemonDTO::from(r).habitat, "unknown");
    }
}

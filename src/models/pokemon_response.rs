#![allow(warnings)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PokemonResponse {
    pub base_happiness: i64,
    pub capture_rate: i64,
    pub color: Item,
    pub egg_groups: Vec<Item>,
    pub evolution_chain: EvolutionChain,
    pub evolves_from_species: Option<Item>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub form_descriptions: Vec<Option<serde_json::Value>>,
    pub forms_switchable: bool,
    pub gender_rate: i64,
    pub genera: Vec<Genus>,
    pub generation: Item,
    pub growth_rate: Item,
    pub habitat: Option<Item>,
    pub has_gender_differences: bool,
    pub hatch_counter: i64,
    pub id: i64,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub name: String,
    pub names: Vec<Name>,
    pub order: i64,
    pub pal_park_encounters: Vec<PalParkEncounter>,
    pub pokedex_numbers: Vec<PokedexNumber>,
    pub shape: Item,
    pub varieties: Vec<Variety>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct EvolutionChain {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Item,
    pub version: Item,
}

#[derive(Serialize, Deserialize)]
pub struct Genus {
    pub genus: String,
    pub language: Item,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    pub language: Item,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PalParkEncounter {
    pub area: Item,
    pub base_score: i64,
    pub rate: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PokedexNumber {
    pub entry_number: i64,
    pub pokedex: Item,
}

#[derive(Serialize, Deserialize)]
pub struct Variety {
    pub is_default: bool,
    pub pokemon: Item,
}

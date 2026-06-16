mod models;

use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::json;

use crate::models::{
    dtos::pokemon_dto::PokemonDTO, pokemon_response::PokemonResponse,
    translation_response::TranslationResponse,
};

pub const POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon-species";
pub const TRANSLATION_API_BASE_URL: &str = "https://api.funtranslations.mercxry.me/v1/translate";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pokemon/{name}", get(search_pokemon_handler))
        .route(
            "/pokemon/translated/{name}",
            get(search_pokemon_translated_handler),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_pokemon(name: String) -> Result<PokemonResponse, reqwest::Error> {
    reqwest::get(format!("{POKE_API_BASE_URL}/{name}"))
        .await?
        .error_for_status()?
        .json::<PokemonResponse>()
        .await
}

async fn search_pokemon_handler(Path(name): Path<String>) -> Result<Json<PokemonDTO>, Response> {
    let pokemon = search_pokemon(name)
        .await
        .map_err(|err| reqwest_err(&err))?;

    Ok(Json(PokemonDTO::from(pokemon)))
}

async fn search_pokemon_translated_handler(
    Path(name): Path<String>,
) -> Result<Json<PokemonDTO>, Response> {
    let mut pokemon = PokemonDTO::from(
        search_pokemon(name)
            .await
            .map_err(|err| reqwest_err(&err))?,
    );

    let translation_type = if pokemon.is_legendary || pokemon.habitat == "Cave" {
        "yoda"
    } else {
        "shakespeare"
    };

    let url = format!("{TRANSLATION_API_BASE_URL}/{translation_type}");
    let body = json!({"text": pokemon.description});

    let response = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|err| reqwest_err(&err))?
        .error_for_status()
        .map_err(|err| reqwest_err(&err))?
        .json::<TranslationResponse>()
        .await
        .map_err(|err| reqwest_err(&err))?;

    pokemon.description = response.contents.translated;

    Ok(Json(pokemon))
}

fn reqwest_err(e: &reqwest::Error) -> Response {
    e.status()
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        .into_response()
}

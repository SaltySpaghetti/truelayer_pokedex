mod models;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::models::{dtos::pokemon_dto::PokemonDTO, pokemon_response::PokemonResponse};

pub const POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon-species";

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();
    let app = Router::new()
        .route("/pokemon/{name}", get(search_pokemon))
        .with_state(http_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_pokemon(
    State(http_client): State<reqwest::Client>,
    Path(name): Path<String>,
) -> Result<Json<PokemonDTO>, Response> {
    let url = format!("{POKE_API_BASE_URL}/{name}");

    let response = http_client
        .get(url)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|err| reqwest_err(&err))?
        .json::<PokemonResponse>()
        .await
        .map_err(|err| reqwest_err(&err))?;

    Ok(Json(PokemonDTO::from(response)))
}

fn reqwest_err(e: &reqwest::Error) -> Response {
    e.status()
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        .into_response()
}

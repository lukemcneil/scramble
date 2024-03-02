mod dictionary;
mod types;

use std::collections::HashMap;
use std::sync::Mutex;

use dictionary::{get_random_letters, Dictionary};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use types::{Game, Games, Player};

use crate::types::{Answer, PlayerData, Result};
use rocket::serde::json::Json;
use rocket::State;

#[macro_use]
extern crate rocket;

#[put("/game/<game_id>", data = "<player>")]
fn create_game(game_id: &str, player: Json<PlayerData>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    games.create(game_id.to_string(), player.into_inner().player)
}

#[post("/game/<game_id>", data = "<player>")]
fn join_game(game_id: &str, player: Json<PlayerData>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    game.add_player(player.into_inner().player)
}

#[get("/game/<game_id>")]
fn game(game_id: &str, games: &State<Mutex<Games>>) -> Result<Json<Game>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    Ok(Json(game.clone()))
}

#[post("/game/<game_id>/answer", data = "<answer>")]
fn answer(
    game_id: &str,
    answer: Json<Answer>,
    games: &State<Mutex<Games>>,
    dictionary: &State<Dictionary>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    game.answer(answer.into_inner(), dictionary)?;
    game.add_round_if_complete(get_random_letters(7));
    Ok(())
}

#[delete("/game/<game_id>/exit", data = "<player>")]
fn exit_game(game_id: &str, player: Json<PlayerData>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    game.remove_player(player.into_inner().player)
}

#[delete("/game/<game_id>")]
fn delete_game(game_id: &str, games: &State<Mutex<Games>>) {
    let mut games = games.lock().unwrap();
    games.delete(game_id)
}

#[get("/game/<game_id>/score")]
fn get_score(
    game_id: &str,
    games: &State<Mutex<Games>>,
    dictionary: &State<Dictionary>,
) -> Result<Json<HashMap<Player, u32>>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?.clone();
    Ok(Json(game.get_score(dictionary)))
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Put,
                Method::Delete,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);
    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount(
            "/",
            routes![
                create_game,
                join_game,
                game,
                answer,
                exit_game,
                delete_game,
                get_score
            ],
        )
        .manage(Mutex::new(Games::default()))
        .manage(Dictionary::new("word-list.txt"))
}

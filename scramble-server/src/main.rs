mod types;

use std::collections::HashMap;
use std::sync::Mutex;

use types::{Game, Games, Player};

use crate::types::{Answer, PlayerData, Result};
use rocket::serde::json::Json;
use rocket::State;

#[macro_use]
extern crate rocket;

#[put("/game/<game_id>", data = "<player>")]
fn create_game(game_id: &str, player: Json<PlayerData>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let player = player.into_inner();
    games.create(game_id.to_string(), player.player)
}

#[post("/game/<game_id>", data = "<player>")]
fn join_game(game_id: &str, player: Json<PlayerData>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    let player = player.into_inner();
    game.add_player(player.player)
}

#[get("/game/<game_id>")]
fn game(game_id: &str, games: &State<Mutex<Games>>) -> Result<Json<Game>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    Ok(Json(game.clone()))
}

#[post("/game/<game_id>/answer", data = "<answer>")]
fn answer(game_id: &str, answer: Json<Answer>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    let answer = answer.into_inner();
    game.answer(answer)?;
    game.add_round_if_complete(Games::get_random_letters());
    Ok(())
}

#[delete("/game/<game_id>/exit", data = "<player>")]
fn exit_game(game_id: &str, player: Json<PlayerData>, games: &State<Mutex<Games>>) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    let player = player.into_inner();
    game.remove_player(player.player)
}

#[delete("/game/<game_id>")]
fn delete_game(game_id: &str, games: &State<Mutex<Games>>) {
    let mut games = games.lock().unwrap();
    games.delete(game_id)
}

#[get("/game/<game_id>/score")]
fn get_score(game_id: &str, games: &State<Mutex<Games>>) -> Result<Json<HashMap<Player, i32>>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?.clone();
    Ok(Json(game.get_score()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
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
}

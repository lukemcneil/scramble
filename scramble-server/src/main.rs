mod dictionary;
mod types;

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;

use dictionary::Dictionary;
use rocket::config::LogLevel;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use structopt::StructOpt;
use types::{CreateGameData, Game, Games, Player};

use crate::types::{Answer, PlayerData, Result};
use rocket::serde::json::Json;
use rocket::{Config, State};

#[macro_use]
extern crate rocket;

#[put("/game/<game_id>", data = "<create_game_data>")]
fn create_game(
    game_id: &str,
    create_game_data: Json<CreateGameData>,
    games: &State<Mutex<Games>>,
    dictionary: &State<Dictionary>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    games.create(
        game_id.to_string(),
        create_game_data.player.clone(),
        dictionary,
        create_game_data.settings.clone(),
    )
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
    game.add_round_if_complete(
        dictionary.get_random_letters(game.settings.number_of_tiles as usize),
        dictionary,
    );
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

#[derive(Debug, StructOpt)]
struct Opt {
    /// An IP address the application will listen on.
    #[structopt(long = "host", short = "H", default_value = "0.0.0.0")]
    address: IpAddr,
    /// A port number to listen on.
    #[structopt(long = "port", short = "P", default_value = "8172")]
    port: u16,
    /// The log level.
    #[structopt(
        default_value = "normal",
        long = "log-level",
        possible_values = &["off", "debug", "normal", "critical"]
    )]
    log_level: LogLevel,
}

#[launch]
fn rocket() -> _ {
    let opt = Opt::from_args();
    let config = Config {
        address: opt.address,
        port: opt.port,
        log_level: opt.log_level,
        ..Config::default()
    };

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
        .configure(config)
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

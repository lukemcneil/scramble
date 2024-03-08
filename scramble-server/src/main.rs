mod dictionary;
mod types;

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use dictionary::Dictionary;
use rocket::config::LogLevel;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use structopt::StructOpt;
use types::{CreateGameData, Game, Games, Player};

use crate::types::{Answer, PlayerData, Result};
use rocket::serde::json::Json;
use rocket::{tokio, Config, State};

#[macro_use]
extern crate rocket;

#[put("/game/<game_id>", data = "<create_game_data>")]
fn create_game(
    game_id: &str,
    create_game_data: Json<CreateGameData>,
    games_state: &State<Arc<Mutex<Games>>>,
    dictionary: &State<Arc<Dictionary>>,
) -> Result<()> {
    let mut games = games_state.lock().unwrap();
    let tiles = dictionary.get_random_letters(create_game_data.settings.number_of_tiles as usize);
    games.create(
        game_id.to_string(),
        create_game_data.player.clone(),
        create_game_data.settings.clone(),
        tiles.clone(),
    )?;
    let dictionary_clone = dictionary.inner().clone();
    let games_state_clone = games_state.inner().clone();
    let game_id_clone = game_id.to_string();
    tokio::spawn(async move {
        get_best_words_for_round(dictionary_clone, tiles, games_state_clone, game_id_clone, 0).await
    });
    Ok(())
}

#[post("/game/<game_id>", data = "<player>")]
fn join_game(
    game_id: &str,
    player: Json<PlayerData>,
    games: &State<Arc<Mutex<Games>>>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    game.add_player(player.into_inner().player)
}

#[get("/game/<game_id>")]
fn game(game_id: &str, games: &State<Arc<Mutex<Games>>>) -> Result<Json<Game>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    Ok(Json(game.clone()))
}

#[post("/game/<game_id>/answer", data = "<answer>")]
fn answer(
    game_id: &str,
    answer: Json<Answer>,
    games_state: &State<Arc<Mutex<Games>>>,
    dictionary: &State<Arc<Dictionary>>,
) -> Result<()> {
    let mut games = games_state.lock().unwrap();
    let game = games.get(game_id)?;
    game.answer(answer.into_inner(), dictionary)?;
    let tiles = dictionary.get_random_letters(game.settings.number_of_tiles as usize);
    if game.add_round_if_complete(tiles.clone()) {
        let dictionary_clone = dictionary.inner().clone();
        let games_state_clone = games_state.inner().clone();
        let game_id_clone = game_id.to_string();
        let i = game.rounds.len() - 1;
        tokio::spawn(async move {
            get_best_words_for_round(dictionary_clone, tiles, games_state_clone, game_id_clone, i)
                .await
        });
    }
    Ok(())
}

#[delete("/game/<game_id>/exit", data = "<player>")]
fn exit_game(
    game_id: &str,
    player: Json<PlayerData>,
    games: &State<Arc<Mutex<Games>>>,
) -> Result<()> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?;
    game.remove_player(player.into_inner().player)
}

#[delete("/game/<game_id>")]
fn delete_game(game_id: &str, games: &State<Arc<Mutex<Games>>>) {
    let mut games = games.lock().unwrap();
    games.delete(game_id)
}

#[get("/game/<game_id>/score")]
fn get_score(
    game_id: &str,
    games: &State<Arc<Mutex<Games>>>,
    dictionary: &State<Arc<Dictionary>>,
) -> Result<Json<HashMap<Player, u32>>> {
    let mut games = games.lock().unwrap();
    let game = games.get(game_id)?.clone();
    Ok(Json(game.get_score(dictionary)))
}

async fn get_best_words_for_round(
    dictionary: Arc<Dictionary>,
    tiles: Vec<char>,
    games: Arc<Mutex<Games>>,
    game_id: String,
    round_number: usize,
) -> Option<()> {
    let best_answers = dictionary.get_best_words(&tiles, 5).await;
    let mut games = games.lock().unwrap();
    let game = games.get(&game_id).ok()?;
    let round = game.rounds.get_mut(round_number)?;
    round.best_answers = best_answers;
    Some(())
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
        .manage(Arc::new(Mutex::new(Games::default())))
        .manage(Arc::new(Dictionary::new("word-list.txt")))
}

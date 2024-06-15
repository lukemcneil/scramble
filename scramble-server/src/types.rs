use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::dictionary::{Dictionary, WordInfo};

pub(crate) type Result<T> = std::result::Result<T, Error>;
// Convert our custom Error type into HTTP responses
impl<'r> Responder<'r, 'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let body = BadRequest::new(self);
        let body = serde_json::to_string(&body).expect("to BadRequest serialize");
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(None, Cursor::new(body))
            .finalize())
    }
}

pub(crate) type Player = String;

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    PlayerConflict,
    PlayerNotFound,
    RoundNotInStartState,
    RoundNotInCollectingAnswersState,
    WordNotInDictionary,
    WordUsesExtraLetters,
    InvalidGameSettings,
    WordMustBeAtLeastTwoLetters,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameConflict => write!(f, "game conflict"),
            Self::GameNotFound => write!(f, "game not found"),
            Self::PlayerConflict => write!(f, "player conflict"),
            Self::PlayerNotFound => write!(f, "player not found"),
            Self::RoundNotInStartState => write!(f, "round not in start state"),
            Self::RoundNotInCollectingAnswersState => {
                write!(f, "round not in collecting answer state")
            }
            Self::WordNotInDictionary => write!(f, "word was not in dictionary"),
            Self::WordUsesExtraLetters => write!(f, "word uses extra letters"),
            Self::InvalidGameSettings => write!(f, "invalid game settings"),
            Self::WordMustBeAtLeastTwoLetters => {
                write!(f, "word must be at least two letters long")
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BadRequest {
    error: String,
    message: String,
}

impl BadRequest {
    fn new(error: Error) -> Self {
        Self {
            error: format!("{error:?}"),
            message: format!("{error}"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PlayerData {
    /// The player with which the request is associated
    pub(crate) player: Player,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CreateGameData {
    /// The player with which the request is associated
    pub(crate) player: Player,
    /// The settings to create the game with
    pub(crate) settings: GameSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct Answer {
    /// The player who gave the answer
    player: Player,
    /// The word the player spelled for the round
    pub answer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct AnswerWithWordInfo {
    /// The player who gave the answer
    player: Player,
    /// The word the player spelled for the round
    pub answer: String,
    /// The score of the word
    pub score: u32,
    /// The definition of the word
    pub definition: String,
}

#[derive(PartialEq)]
pub(crate) enum RoundState {
    Start,
    CollectingAnswers,
    Complete,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The list of letters that can be used to spell a word
    pub(crate) letters: Vec<char>,
    /// The list of answers given, one per player
    pub(crate) answers: Vec<AnswerWithWordInfo>,
    /// The number of lookups that a player has used
    pub(crate) lookups_used: HashMap<Player, u32>,
    /// The list of best answers for this round
    pub(crate) best_answers: Vec<WordInfo>,
}

impl Round {
    fn new(letters: Vec<char>) -> Self {
        Round {
            letters: letters.clone(),
            answers: Vec::new(),
            lookups_used: HashMap::new(),
            best_answers: Vec::new(),
        }
    }

    fn state(&self, players: usize) -> RoundState {
        if self.answers.is_empty() {
            RoundState::Start
        } else if self.answers.len() < players {
            RoundState::CollectingAnswers
        } else if self.answers.len() == players {
            RoundState::Complete
        } else {
            panic!("Round in unknown state")
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct GameSettings {
    /// The number of tiles to make words from
    pub(crate) number_of_tiles: u32,
    /// The number of lookups allowed before forfeiting turn
    pub(crate) number_of_lookups: u32,
    /// The method to score words
    pub(crate) scoring_method: ScoringMethod,
    /// Letters that will not show up
    pub(crate) banned_letters: HashSet<char>,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            number_of_tiles: 7,
            number_of_lookups: 2,
            scoring_method: ScoringMethod::Normal,
            banned_letters: HashSet::new(),
        }
    }
}

impl GameSettings {
    fn is_valid(&self) -> bool {
        self.number_of_tiles >= 2 
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) enum ScoringMethod {
    Normal,
    Length,
}

impl ScoringMethod {
    pub(crate) fn score(&self, word_info: &WordInfo) -> u32 {
        match self {
            ScoringMethod::Normal => word_info.score,
            ScoringMethod::Length => word_info.word.len() as u32,
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub(crate) players: HashSet<String>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub(crate) rounds: Vec<Round>,
    /// The settings for the game
    pub(crate) settings: GameSettings,
}

impl Game {
    pub(crate) fn add_player(&mut self, player: Player) -> Result<()> {
        // Only allow adding players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        if self.players.insert(player) {
            Ok(())
        } else {
            Err(Error::PlayerConflict)
        }
    }

    pub(crate) fn remove_player(&mut self, player: Player) -> Result<()> {
        // Only allow removing players at the start of a round
        if self.current_round_state() != RoundState::Start {
            return Err(Error::RoundNotInStartState);
        }
        self.players.remove(&player);
        Ok(())
    }

    pub(crate) fn answer(&mut self, answer: Answer, dictionary: &Dictionary) -> Result<()> {
        let player = &answer.player;
        // Confirm the player exists
        if !self.players.contains(player) {
            return Err(Error::PlayerNotFound);
        }
        // Confirm we are collecting answers for the current round
        let state = self.current_round_state();
        if state != RoundState::Start && self.current_round_state() != RoundState::CollectingAnswers
        {
            return Err(Error::RoundNotInCollectingAnswersState);
        }

        let number_of_lookups = self.settings.number_of_lookups;
        let scoring_method = self.settings.scoring_method.clone();
        let round = self.current_round_mut();
        // Check if this player already added an answer
        for a in &round.answers {
            if a.player == answer.player {
                return Ok(());
            }
        }
        // Check that the word is at least 2 letters long
        if answer.answer.len() < 2 {
            return Err(Error::WordMustBeAtLeastTwoLetters);
        }
        // Check that the word is valid with the letters from this round
        if !Dictionary::check_word_uses_letters(&round.letters, &answer.answer) {
            return Err(Error::WordUsesExtraLetters);
        }
        // Check if the word is playable
        match dictionary.get_word_info_if_playable(&answer.answer) {
            Some(word_info) => {
                let score = scoring_method.score(word_info);

                let answer_with_info = AnswerWithWordInfo {
                    player: answer.player,
                    answer: answer.answer,
                    score,
                    definition: word_info.definition.clone(),
                };
                // Add the answer with info
                round.answers.push(answer_with_info);
                Ok(())
            }
            None => {
                let lookups_used = round.lookups_used.entry(player.clone()).or_default();
                *lookups_used += 1;
                if *lookups_used == number_of_lookups + 1 {
                    let empty_answer = AnswerWithWordInfo {
                        player: answer.player,
                        answer: answer.answer,
                        score: 0,
                        definition: String::from(""),
                    };
                    round.answers.push(empty_answer);
                    Ok(())
                } else {
                    Err(Error::WordNotInDictionary)
                }
            }
        }
    }

    pub(crate) fn add_round_if_complete(&mut self, letters: Vec<char>) -> bool {
        if self.current_round_state() == RoundState::Complete {
            self.add_round(letters);
            true
        } else {
            false
        }
    }

    fn add_round(&mut self, letters: Vec<char>) {
        self.rounds.push(Round::new(letters));
    }

    pub(crate) fn current_round(&self) -> &Round {
        let index = self.rounds.len() - 1;
        &self.rounds[index]
    }

    fn current_round_mut(&mut self) -> &mut Round {
        let index = self.rounds.len() - 1;
        &mut self.rounds[index]
    }

    fn current_round_state(&self) -> RoundState {
        let players = self.players.len();
        let round = self.current_round();
        round.state(players)
    }

    pub fn get_score(
        &self,
        dictionary: &Dictionary,
        scoring_method: &ScoringMethod,
    ) -> HashMap<String, u32> {
        let mut scores = HashMap::new();
        for round in &self.rounds {
            for answer in round.answers.iter() {
                let score = scores.entry(answer.player.clone()).or_insert(0);
                if let Some(word_info) = dictionary.get_word_info_if_playable(&answer.answer) {
                    *score += scoring_method.score(word_info);
                }
            }
        }
        scores
    }
}

#[derive(Default)]
pub(crate) struct Games(HashMap<String, Game>);

impl Games {
    #[allow(clippy::map_entry)]
    pub(crate) fn create(
        &mut self,
        game_id: String,
        initial_player: Player,
        settings: GameSettings,
        letters: Vec<char>,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            if !settings.is_valid() {
                return Err(Error::InvalidGameSettings);
            }
            let mut game = Game {
                settings,
                ..Default::default()
            };
            game.add_round(letters);
            game.add_player(initial_player)?;
            self.0.insert(game_id, game);
            Ok(())
        }
    }

    pub(crate) fn get(&mut self, game_id: &str) -> Result<&mut Game> {
        self.0.get_mut(game_id).ok_or(Error::GameNotFound)
    }

    pub(crate) fn delete(&mut self, game_id: &str) {
        self.0.remove(game_id);
    }
}

#[test]
fn test_get_score() -> Result<()> {
    let mut game = Game::default();
    let dictionary = Dictionary::new("word-list.txt");
    game.add_round(vec!['S', 'C', 'R', 'A', 'M', 'B', 'L', 'E']);
    game.add_player(String::from("test"))?;
    assert!(game
        .answer(
            Answer {
                player: String::from("test"),
                answer: String::from("scr"),
            },
            &dictionary,
        )
        .is_err_and(|e| matches!(e, Error::WordNotInDictionary)));
    assert!(game
        .answer(
            Answer {
                player: String::from("test"),
                answer: String::from("bell"),
            },
            &dictionary,
        )
        .is_err_and(|e| matches!(e, Error::WordUsesExtraLetters)));
    game.answer(
        Answer {
            player: String::from("test"),
            answer: String::from("scramble"),
        },
        &dictionary,
    )?;
    let mut expected = HashMap::new();
    expected.insert(String::from("test"), 14);
    assert_eq!(
        game.get_score(&dictionary, &ScoringMethod::Normal),
        expected
    );
    Ok(())
}

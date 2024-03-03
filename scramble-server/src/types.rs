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

use crate::dictionary::{get_random_letters, Dictionary, WordInfo};

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
            Self::WordNotInDictionary => write!(f, "word is not in dictionary"),
            Self::WordUsesExtraLetters => write!(f, "word uses extra letters"),
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
    /// The list of best answers for this round
    pub(crate) best_answers: Vec<WordInfo>,
}

impl Round {
    fn new(letters: Vec<char>, dictionary: &Dictionary) -> Self {
        Round {
            letters: letters.clone(),
            answers: Vec::new(),
            best_answers: dictionary.get_best_words(&letters, 5),
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

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub(crate) players: HashSet<String>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub(crate) rounds: Vec<Round>,
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

        let round = self.current_round_mut();
        // Check if this player already added an answer
        for a in &round.answers {
            if a.player == answer.player {
                return Ok(());
            }
        }
        // Check if the word is playable
        match dictionary.get_word_info_if_playable(&answer.answer) {
            Some(word_info) => {
                // Check that the word is valid with the letters from this round
                if !Dictionary::check_word_uses_letters(&round.letters, &answer.answer) {
                    return Err(Error::WordUsesExtraLetters);
                }

                let answer_with_info = AnswerWithWordInfo {
                    player: answer.player,
                    answer: answer.answer,
                    score: word_info.score,
                    definition: word_info.definition.clone(),
                };
                // Add the answer with info
                round.answers.push(answer_with_info);
                Ok(())
            }
            None => Err(Error::WordNotInDictionary),
        }
    }

    pub(crate) fn add_round_if_complete(&mut self, letters: Vec<char>, dictionary: &Dictionary) {
        if self.current_round_state() == RoundState::Complete {
            self.add_round(letters, dictionary);
        }
    }

    fn add_round(&mut self, letters: Vec<char>, dictionary: &Dictionary) {
        self.rounds.push(Round::new(letters, dictionary));
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

    pub fn get_score(&self, dictionary: &Dictionary) -> HashMap<String, u32> {
        let mut scores = HashMap::new();
        for round in &self.rounds {
            for answer in round.answers.iter() {
                let score = scores.entry(answer.player.clone()).or_insert(0);
                let word_score = dictionary
                    .get_word_info_if_playable(&answer.answer)
                    .expect("answers should all be in dictionary")
                    .score;
                *score += word_score;
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
        dictionary: &Dictionary,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
            game.add_round(get_random_letters(7), dictionary);
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
    game.add_round(vec!['S', 'C', 'R', 'A', 'M', 'B', 'L', 'E'], &dictionary);
    game.add_player(String::from("test"))?;
    assert!(game
        .answer(
            Answer {
                player: String::from("test"),
                answer: String::from("notaword"),
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
    assert_eq!(game.get_score(&dictionary), expected);
    Ok(())
}

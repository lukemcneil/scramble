use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use rand::{seq::SliceRandom, thread_rng};
use rocket::tokio;
use serde::{Deserialize, Serialize};

use crate::types::ScoringMethod;

struct Tile {
    letter: char,
    points: u32,
    count: u32,
}

macro_rules! new_tile {
    ($letter:expr, $points:expr, $count:expr) => {
        Tile {
            letter: $letter,
            points: $points,
            count: $count,
        }
    };
}

#[test]
fn test_tiles() {
    let num_tiles: u32 = TILES.iter().map(|t| t.count).sum();
    assert_eq!(num_tiles, 98);
    let total_points: u32 = TILES.iter().map(|t| t.points * t.count).sum::<u32>();
    assert_eq!(total_points, 187);
}

const TILES: [Tile; 26] = [
    // new_tile!('?', 0, 2),
    new_tile!('E', 1, 12),
    new_tile!('A', 1, 9),
    new_tile!('I', 1, 9),
    new_tile!('O', 1, 8),
    new_tile!('N', 1, 6),
    new_tile!('R', 1, 6),
    new_tile!('T', 1, 6),
    new_tile!('L', 1, 4),
    new_tile!('S', 1, 4),
    new_tile!('U', 1, 4),
    new_tile!('D', 2, 4),
    new_tile!('G', 2, 3),
    new_tile!('B', 3, 2),
    new_tile!('C', 3, 2),
    new_tile!('M', 3, 2),
    new_tile!('P', 3, 2),
    new_tile!('F', 4, 2),
    new_tile!('H', 4, 2),
    new_tile!('V', 4, 2),
    new_tile!('W', 4, 2),
    new_tile!('Y', 4, 2),
    new_tile!('K', 5, 1),
    new_tile!('J', 8, 1),
    new_tile!('X', 8, 1),
    new_tile!('Q', 10, 1),
    new_tile!('Z', 10, 1),
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WordInfo {
    pub word: String,
    pub score: u32,
    pub definition: String,
}

pub struct Dictionary {
    playable_words: HashMap<String, WordInfo>,
    letter_scores: HashMap<char, u32>,
    all_tiles: Vec<char>,
}

impl Dictionary {
    pub fn new(path: &str) -> Self {
        let all_tiles = TILES
            .iter()
            .flat_map(|tile| vec![tile.letter; tile.count as usize])
            .collect();
        let mut words = Self {
            playable_words: HashMap::new(),
            letter_scores: TILES.iter().map(|t| (t.letter, t.points)).collect(),
            all_tiles,
        };
        words.playable_words = words.read_words(path);
        words
    }

    pub(crate) fn get_random_letters(&self, size: usize) -> Vec<char> {
        let mut rng = thread_rng();
        loop {
            let mut all_tiles = self.all_tiles.clone();
            all_tiles.shuffle(&mut rng);
            all_tiles.truncate(size);
            if self.has_playable_word(&all_tiles) {
                return all_tiles;
            }
        }
    }

    pub fn get_word_info_if_playable(&self, s: &str) -> Option<&WordInfo> {
        self.playable_words.get(&s.to_ascii_uppercase())
    }

    fn read_words(&self, path: &str) -> HashMap<String, WordInfo> {
        let mut words = HashMap::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let (word, definition) = line.split_once('\t').unwrap();
            let word = word.to_string();
            let definition = definition.to_string();
            let score = self.calculate_score(&word);
            words.insert(
                word.clone(),
                WordInfo {
                    word,
                    score,
                    definition,
                },
            );
        }
        words
    }

    fn calculate_score(&self, s: &str) -> u32 {
        s.chars()
            .filter_map(|c| self.letter_scores.get(&c.to_ascii_uppercase()))
            .sum()
    }

    pub fn check_word_uses_letters(letters: &[char], answer: &str) -> bool {
        let mut letters_left: HashMap<char, u32> = HashMap::new();
        for letter in letters {
            let letter_count = letters_left.entry(*letter).or_default();
            *letter_count += 1;
        }
        for letter in answer.chars() {
            let letter = letter.to_ascii_uppercase();
            let entry = letters_left.entry(letter);
            match entry {
                std::collections::hash_map::Entry::Occupied(mut letter_count) => {
                    let letter_count = letter_count.get_mut();
                    if letter_count > &mut 0 {
                        *letter_count -= 1;
                    } else {
                        return false;
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {
                    return false;
                }
            }
        }
        true
    }

    pub async fn get_best_words(
        &self,
        letters: &[char],
        num_words: usize,
        scoring_method: &ScoringMethod,
    ) -> Vec<WordInfo> {
        let mut best_words: Vec<WordInfo> = Vec::new();

        let mut i = 0;
        for (word, info) in &self.playable_words {
            i += 1;
            if i == 1000 {
                i = 0;
                tokio::task::yield_now().await;
            }
            if Self::check_word_uses_letters(letters, word) {
                let mut info = info.clone();
                if matches!(scoring_method, ScoringMethod::Length) {
                    info.score = info.word.len() as u32;
                }
                best_words.push(info);
            }
        }
        best_words.sort_by(|a, b| {
            let b_score = scoring_method.score(b);
            let a_score = scoring_method.score(a);
            b_score.cmp(&a_score)
        });
        best_words.truncate(num_words);
        best_words
    }

    fn has_playable_word(&self, letters: &[char]) -> bool {
        self.playable_words
            .iter()
            .any(|(word, _)| Self::check_word_uses_letters(letters, word))
    }
}

#[test]
fn test_read_words() {
    let words = Dictionary::new("word-list.txt");
    assert_eq!(words.get_word_info_if_playable("zeugma").unwrap().score, 18);
    assert!(words.get_word_info_if_playable("notaword").is_none());
}

#[tokio::test]
async fn test_best_words() {
    let words = Dictionary::new("word-list.txt");
    for value in words
        .get_best_words(
            &['R', 'E', 'M', 'O', 'R', 'S', 'E'],
            5,
            &ScoringMethod::Normal,
        )
        .await
    {
        println!("{:?}", value);
    }
}

#[tokio::test]
async fn test_scrabble_probability() {
    let words = Dictionary::new("word-list.txt");
    let n = 10;
    let mut scrabbles = 0;
    let mut no_words = 0;
    for _ in 0..n {
        let letters = words.get_random_letters(7);
        let best_words = words.get_best_words(&letters, 1, &ScoringMethod::Normal);
        if let Some(best_word) = best_words.await.first() {
            println!("best word len: {}", best_word.word.len());
            if best_word.word.len() == 7 {
                println!("{letters:?}: {best_word:?}");
                scrabbles += 1;
            }
        } else {
            println!("no words from {letters:?}");
            no_words += 1;
        }
    }
    println!("{scrabbles} / {n} scrabbles\n{no_words} / {n} no words");
}

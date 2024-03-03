use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use rand::distributions::{Distribution, WeightedIndex};
use serde::{Deserialize, Serialize};

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

pub(crate) fn get_random_letters(size: usize) -> Vec<char> {
    let counts: Vec<u32> = TILES.iter().map(|tile| tile.count).collect();
    let distribution = WeightedIndex::new(counts).unwrap();
    let mut rng = rand::thread_rng();

    let chosen_indices: Vec<usize> = distribution.sample_iter(&mut rng).take(size).collect();

    chosen_indices
        .into_iter()
        .map(|index| TILES[index].letter)
        .collect()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WordInfo {
    pub word: String,
    pub score: u32,
    pub definition: String,
}

pub struct Dictionary {
    playable_words: HashMap<String, WordInfo>,
    letter_scores: HashMap<char, u32>,
}

impl Dictionary {
    pub fn new(path: &str) -> Self {
        let mut words = Self {
            playable_words: HashMap::new(),
            letter_scores: TILES.iter().map(|t| (t.letter, t.points)).collect(),
        };
        words.playable_words = words.read_words(path);
        words
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

    pub fn get_best_words(&self, letters: &[char], num_words: usize) -> Vec<WordInfo> {
        let mut best_words: Vec<WordInfo> = self
            .playable_words
            .iter()
            .filter_map(|(word, info)| {
                if Self::check_word_uses_letters(letters, &word) {
                    Some(info.clone())
                } else {
                    None
                }
            })
            .collect();
        best_words.sort_by(|a, b| b.score.cmp(&a.score));
        best_words.truncate(num_words);
        best_words
    }
}

#[test]
fn test_read_words() {
    let words = Dictionary::new("word-list.txt");
    assert_eq!(words.get_word_info_if_playable("zeugma").unwrap().score, 18);
    assert!(words.get_word_info_if_playable("notaword").is_none());
}

#[test]
fn test_best_words() {
    let words = Dictionary::new("word-list.txt");
    for value in words.get_best_words(&['R', 'E', 'M', 'O', 'R', 'S', 'E'], 5) {
        println!("{:?}", value);
    }
}

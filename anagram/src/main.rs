use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;

type WordMap = HashMap<String, Vec<String>>;

const DICT_PATH: &str = "/usr/share/dict/words";

fn main() {
    let dict_path = env::var("DICT_PATH").unwrap_or(DICT_PATH.to_string());

    let word = std::env::args().nth(1).expect("Usage: anagram-search word");
    println!("search anagrams for {} from {} ...", word, dict_path);

    let sorted = sort_word(&word);
    let map = get_word_map(&dict_path).unwrap();

    match map.get(&sorted) {
        Some(words) => words.iter().for_each(|w| println!("{}", w)),
        _ => println!("Noop"),
    }
}

fn sort_word(word: &str) -> String {
    let mut s = word.chars().collect::<Vec<char>>();
    s.sort();
    s.into_iter().collect()
}

fn get_word_map(dict_path: &str) -> Result<WordMap, io::Error> {
    let file = File::open(dict_path)?;
    let reader = io::BufReader::new(file);

    let mut word_map = HashMap::new();
    for line in reader.lines() {
        let word = line?;
        let key = sort_word(&word);
        word_map.entry(key).or_insert(Vec::new()).push(word);
    }

    Ok(word_map)
}

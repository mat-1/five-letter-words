use five_letter_words::{get_results, word_to_num};
use std::collections::{BTreeSet, HashSet};

const WORDS: &str = include_str!("wordle-words.txt");
// const WORDS: &str = include_str!("smaller.txt");

fn main() {
    let words: HashSet<&str> = WORDS.lines().collect();
    let packed_words: BTreeSet<u32> = words.iter().copied().flat_map(word_to_num).collect();
    let packed_words: Vec<u32> = packed_words.into_iter().collect();
    // let packed_words: [u32; 5183] = packed_words.try_into().unwrap();

    println!(
        "found {} unique sets of 5 letters that appear in words",
        packed_words.len()
    );
    let results = get_results(&packed_words);

    println!(
        "{} five word anagram groups with no overlapping letters found",
        results.len()
    );
    for (w1, w2, w3, w4, w5) in results {
        for word1 in get_words_for_num(w1, &words) {
            for word2 in get_words_for_num(w2, &words) {
                for word3 in get_words_for_num(w3, &words) {
                    for word4 in get_words_for_num(w4, &words) {
                        for word5 in get_words_for_num(w5, &words) {
                            println!("{word1} {word2} {word3} {word4} {word5}");
                        }
                    }
                }
            }
        }
    }
}

fn get_words_for_num<'a>(num: u32, words: &'a HashSet<&'a str>) -> impl Iterator<Item = &'a str> {
    words
        .iter()
        .copied()
        .filter(move |word| word_to_num(word) == Some(num))
}

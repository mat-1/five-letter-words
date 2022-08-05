use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::collections::BTreeSet;

pub fn get_results(packed_words: &[u32]) -> BTreeSet<(u32, u32, u32, u32, u32)> {
    let results: BTreeSet<(u32, u32, u32, u32, u32)> = packed_words
        .par_iter()
        // .iter()
        .enumerate()
        .flat_map(|(i1, &w1)| {
            let mut results: BTreeSet<(u32, u32, u32, u32, u32)> = BTreeSet::new();
            let ti = i1 + 1;
            let wi = w1;
            let mut packed_words_iter = packed_words[ti..].iter();
            while let Some(&w2) = packed_words_iter.next() {
                if (wi & w2) != 0 {
                    continue;
                }
                let wi = wi | w2;
                let mut packed_words_iter2 = packed_words_iter.clone();
                while let Some(&w3) = packed_words_iter2.next() {
                    if (wi & w3) != 0 {
                        continue;
                    }
                    let wi = wi | w3;
                    let mut packed_words_iter3 = packed_words_iter2.clone();
                    while let Some(&w4) = packed_words_iter3.next() {
                        if (wi & w4) != 0 {
                            continue;
                        }
                        let wi = wi | w4;
                        let packed_words_iter4 = packed_words_iter3.clone();
                        for w5 in packed_words_iter4 {
                            if (wi & w5) == 0 {
                                results.insert((w1, w2, w3, w4, *w5));
                            }
                        }
                    }
                }
            }
            // results
            results.into_par_iter()
        })
        .collect();
    results
}

pub fn word_to_num(word: &str) -> Option<u32> {
    let mut res = 0;
    for c in word.bytes() {
        let letter_bit = 1 << (c - b'a');
        // letter is already in the word
        if (res & letter_bit) != 0 {
            return None;
        }
        res |= letter_bit;
    }
    Some(res)
}

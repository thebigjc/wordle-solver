use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use bit_set::BitSet;
use std::convert::TryInto;

fn load_words(f: &str) -> io::Result<Vec<String>> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

enum Color {
    Grey,
    Yellow,
    Green
}

struct Word {
    w: [u8; 5],
    set : BitSet,
}

impl Word {
    fn new(s : &String) -> Word {
        let w = s.as_bytes();
        let mut set = BitSet::new();
        w.iter().for_each(|x| { set.insert((x - 'a' as u8) as usize); });

        Word {
            w: w.try_into().expect("wrong length"),
            set: set
        }
    }
}

fn make_idx(ac: &[u8], bc: &Word) -> usize {
    let mut idx = 0;
    let mut mul = 1;

    for i in 0..5 {
        idx += if ac[i] == bc.w[i] {
            Color::Green as usize
        } else if bc.set.contains((&ac[i]-'a' as u8) as usize) {
            Color::Yellow as usize
        } else {
            Color::Grey as usize
        } * mul;
        mul *= 3;
    }
    idx
}

const MASK_SIZE : usize = 3 * 3 * 3 * 3 * 3;

fn calc_entropy_for_word(q: &String, word_chars: &Vec<Word>) -> f64 {
    let qc = q.as_bytes();

    let mut mask_map : [u8; MASK_SIZE] = [0; MASK_SIZE];

    for wc in word_chars {
        let idx = make_idx(&qc, &wc);
        mask_map[idx]+= 1;
    }

    let non_zero = mask_map.iter().filter(|x| **x > 0).count() as f64;
    let entropy : f64 = mask_map.iter().filter(|x| **x > 0).map(|x| { 
        let p = *x as f64 / non_zero;
        p * p.log2()
    }).sum();

    -entropy
}

fn get_best_word(words: &Vec<String>, legal_words: &Vec<String>) -> (String, f64) {
    let mut entropy = Vec::new();

    let word_chars: Vec<Word> = words.iter().map(|x| Word::new(x)).collect();

    for q in legal_words.iter() {
        let word_entropy = calc_entropy_for_word(q, &word_chars);
        entropy.push((q, word_entropy));
        //println!("{} has entropy {}", q, word_entropy);
    }

    entropy.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

    /*for (s, f) in &entropy {
        println!("{}: {}", s, f);
    }*/

    let (s, f) = entropy.last().unwrap();
    (s.to_string(), *f)
}

fn main() -> io::Result<()> {
    let words = load_words("words2.txt")?;
    let legal_words = load_words("legal.txt")?;

    //println!("{}", words.len());
    let (first_guess, entrop) = get_best_word(&words, &legal_words);
    println!("let's guess: {} which has entropy: {}", first_guess, entrop);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        let mask = Mask::make(
            "rebut".as_bytes(),
            "rebut".as_bytes()
        );

        assert_eq!(mask, Mask(Color::Green, Color::Green, Color::Green, Color::Green, Color::Green));

        let mask2 = Mask::make(
            &"rebut".as_bytes(),
            &"butch".as_bytes(),
        );

        assert_eq!(mask2, Mask(Color::Grey, Color::Grey, Color::Yellow, Color::Yellow, Color::Yellow));
    }
}

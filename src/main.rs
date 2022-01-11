use rayon::prelude::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn load_words(f: &str) -> io::Result<Vec<String>> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

enum Color {
    Grey,
    Yellow,
    Green,
}

struct Word {
    set: u32,
    w: [u32; 5],
}

impl Word {
    fn new(s: &String) -> Word {
        let w: [u32; 5] = s
            .as_bytes()
            .iter()
            .map(|x| *x as u32)
            .collect::<Vec<u32>>()
            .try_into()
            .expect("wrong size");
        let mut set: u32 = 0;
        w.iter().for_each(|x| set |= 1 << (x - 'a' as u32));

        Word { w, set }
    }
}

fn make_idx(ac: &Word, bc: &Word) -> usize {
    let mut idx = 0;
    let mut mul = 1;

    for i in 0..5 {
        idx += if ac.w[i] == bc.w[i] {
            Color::Green as usize
        } else if bc.set & (1 << (&ac.w[i] - 'a' as u32)) != 0 {
            Color::Yellow as usize
        } else {
            Color::Grey as usize
        } * mul;
        mul *= 3;
    }
    idx
}

const MASK_SIZE: usize = 3 * 3 * 3 * 3 * 3;

fn calc_entropy_for_word(q: &String, word_chars: &Vec<Word>) -> f64 {
    let qc = Word::new(q);

    let mut mask_map: [usize; MASK_SIZE] = [0; MASK_SIZE];

    for wc in word_chars {
        let idx = make_idx(&qc, &wc);
        mask_map[idx] += 1;
    }

    let words = word_chars.len() as f64;

    let entropy: f64 = mask_map
        .iter()
        .map(|x| {
            if *x > 0 {
                let p = *x as f64 / words;
                p * p.log2()
            } else {
                0.0
            }
        })
        .sum();

    -entropy
}

fn get_best_word(words: &Vec<String>, legal_words: &Vec<String>) -> (String, f64) {
    let word_chars: Vec<Word> = words.par_iter().map(|x| Word::new(x)).collect();

    let entropy: Vec<(&String, f64)> = legal_words
        .par_iter()
        .map(|x| (x, calc_entropy_for_word(x, &word_chars)))
        .collect();

    let (s, f) = entropy
        .par_iter()
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .unwrap();

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

    fn test_idx (colors: [Color; 5]) -> usize {
        let mut idx = 0;
        let mut mul = 1;

        for i in colors {
            idx += i as usize * mul;
            mul *= 3;
        };

        idx
    }

    #[test]
    fn test_match() {
        let mask = make_idx(&Word::new(&String::from("rebut")), &Word::new(&String::from("rebut")));

        assert_eq!(
            mask,
            test_idx([Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green])
            );

        let mask2 = make_idx(&Word::new(&String::from("rebut")), &Word::new(&String::from("butch")));

        assert_eq!(
            mask2,
            test_idx(
                [Color::Grey,
                Color::Grey,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow]
            )
        );
    }
}

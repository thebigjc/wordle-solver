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
    w: [usize; 5],
    set: [usize; 26]
}

impl Word {
    fn new(s: &str) -> Word {
        let w: [usize; 5] = s
            .as_bytes()
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<usize>>()
            .try_into()
            .expect("wrong size");

        let mut set: [usize; 26] = [0; 26];

        for i in 0..5 {
            let c = w[i];
            if s.contains(c as u8 as char) {
                set[c - 'a' as usize] += 1;
            }
        }

        Word { w, set }
    }
}

fn make_idx(ac: &Word, bc: &Word) -> usize {
    let mut idx = 0;
    let mut mul = 1;

    let mut b_set = bc.set.clone();

    for i in 0..5 {
        idx += if ac.w[i] == bc.w[i] {
            b_set[ac.w[i] - 'a' as usize] -= 1;
            Color::Green as usize
        } else if b_set[ac.w[i] - 'a' as usize] > 0 {
            b_set[ac.w[i] - 'a' as usize] -= 1;
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

    fn test_idx(colors: [Color; 5]) -> usize {
        let mut idx = 0;
        let mut mul = 1;

        for i in colors {
            idx += i as usize * mul;
            mul *= 3;
        }

        idx
    }

    #[test]
    fn test_green() {
        let mask = make_idx(&Word::new("rebut"), &Word::new("rebut"));

        assert_eq!(
            mask,
            test_idx([
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Green
            ])
        );
    }

    #[test]
    fn test_two_grey_three_yellow() {
        let mask2 = make_idx(&Word::new("rebut"), &Word::new("butch"));

        assert_eq!(
            mask2,
            test_idx([
                Color::Grey,
                Color::Grey,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow
            ])
        );
    }

    #[test]
    fn test_double_letter() {
        let mask3 = make_idx(&Word::new("blood"), &Word::new("proxy"));

        assert_eq!(
            mask3,
            test_idx([
                Color::Grey,
                Color::Grey,
                Color::Green,
                Color::Grey,
                Color::Grey
            ])
        );
    }

    #[test]
    fn test_double_letter2() {
        let mask4 = make_idx(&Word::new("babes"), &Word::new("abbey"));

        assert_eq!(
            mask4,
            test_idx([
                Color::Yellow,
                Color::Yellow,
                Color::Green,
                Color::Green,
                Color::Grey
            ])
        );
    }
}

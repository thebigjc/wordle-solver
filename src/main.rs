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
    set: [usize; 26],
    s: String,
}

impl Word {
    fn new(s: &str) -> Word {
        let w: [usize; 5] = s
            .as_bytes()
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap_or_else(|_| panic!("wrong size: {}", s));

        let mut set: [usize; 26] = [0; 26];

        for i in 0..5 {
            let c = w[i];
            if s.contains(c as u8 as char) {
                set[c - 'a' as usize] += 1;
            }
        }

        Word {
            w: w,
            set: set,
            s: s.to_string(),
        }
    }
}

fn make_idx(ac: &Word, bc: &Word) -> usize {
    let mut idx = 0;

    let mut b_set = bc.set.clone();

    let mut mul = 1;

    for i in 0..5 {
        idx += if ac.w[i] == bc.w[i] {
            b_set[ac.w[i] - 'a' as usize] -= 1;
            Color::Green as usize
        } else {
            Color::Grey as usize
        } * mul;

        mul *= 3;
    }

    mul = 1;

    for i in 0..5 {
        idx += if ac.w[i] != bc.w[i] && b_set[ac.w[i] - 'a' as usize] > 0 {
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

fn calc_entropy_for_words(
    legal_words: &Vec<&Word>,
    word_chars: &Vec<&Word>,
    depth: usize,
) -> Vec<(String, f64)> {
    legal_words
        .par_iter()
        .map(|x| {
            (
                x.s.clone(),
                calc_entropy_for_word(x, legal_words, word_chars, depth),
            )
        })
        .collect()
}

fn calc_entropy_for_word(
    q: &Word,
    legal_words: &Vec<&Word>,
    word_chars: &Vec<&Word>,
    depth: usize,
) -> f64 {
    if depth > 0 {
        println!("Depth search starting from {}", q.s);
    }
    let mut mask_map: [Vec<&Word>; MASK_SIZE] = [(); MASK_SIZE].map(|_| Vec::<&Word>::new());

    for w in word_chars {
        let idx = make_idx(&q, &w);
        mask_map[idx].push(w);
    }

    let words = word_chars.len() as f64;

    let legal_without: Vec<&Word> = legal_words
        .iter()
        .filter(|x| x.s != q.s)
        .map(|x| *x)
        .collect();

    let entropy: f64 = mask_map
        .par_iter()
        .map(|x| {
            let l = x.len();
            if l > 0 {
                let p = l as f64 / words;
                let r = p * p.log2();
                if l < word_chars.len() && l > 1 && depth > 0 {
                    let deep_entropy = calc_entropy_for_words(&legal_without, x, depth - 1);
                    let mut sums = 0.0;
                    let mut c = 0.0;
                    for (_, f) in deep_entropy.iter() {
                        sums += f;
                        c += 1.0;
                    }
                    r - (sums / c)
                } else {
                    r
                }
            } else {
                0.0
            }
        })
        .sum::<f64>()
        * -1.0;

    if depth > 0 {
        println!("{} - {} : {}", depth, entropy, q.s);
    }

    entropy
}

fn get_best_word(words: &Vec<String>, legal_words: &Vec<String>) -> (String, f64) {
    let word_chars: Vec<Word> = words.iter().map(|x| Word::new(x)).collect();
    let legal_chars: Vec<Word> = legal_words.iter().map(|x| Word::new(x)).collect();

    let word_chars_ref: Vec<&Word> = word_chars.iter().map(|x| x).collect();
    let legal_chars_ref: Vec<&Word> = legal_chars.iter().map(|x| x).collect();

    let mut legal_entropy = calc_entropy_for_words(&legal_chars_ref, &word_chars_ref, 0);

    legal_entropy.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

    let top10_chars: Vec<Word> = legal_entropy
        .iter()
        .take(10)
        .map(|(s, _)| Word::new(s))
        .collect();
    let top10_chars_ref: Vec<&Word> = top10_chars.iter().map(|x| x).collect();

    let mut deep_entropy: Vec<(String, f64)> = top10_chars_ref
        .iter()
        .map(|x| {
            (
                x.s.clone(),
                calc_entropy_for_word(x, &legal_chars_ref, &word_chars_ref, 1),
            )
        })
        .collect();

    deep_entropy.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

    let (s, f) = deep_entropy.first().unwrap();

    (s.clone(), *f)
}

fn main() -> io::Result<()> {
    let words = load_words("words2.txt")?;
    let legal_words = load_words("legal.txt")?;

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

    #[test]
    fn test_overflow() {
        let mask4 = make_idx(&Word::new("malax"), &Word::new("knead"));

        assert_eq!(
            mask4,
            test_idx([
                Color::Grey,
                Color::Grey,
                Color::Grey,
                Color::Green,
                Color::Grey
            ])
        );
    }

    #[test]
    fn test_index_oob() {
        let mask4 = make_idx(&Word::new("tepoy"), &Word::new("coyly"));

        assert_eq!(
            mask4,
            test_idx([
                Color::Grey,
                Color::Grey,
                Color::Grey,
                Color::Yellow,
                Color::Green
            ])
        );
    }
}

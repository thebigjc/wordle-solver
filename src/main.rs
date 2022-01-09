use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

enum Color {
    Grey,
    Yellow,
    Green
}

fn load_words(f: &str) -> io::Result<Vec<String>> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn make_mask(ac: &Vec<char>, bc: &Vec<char>, mask: &mut [u8; 5]) {
    for i in 0..mask.len() {
        if ac[i] == bc[i] {
            mask[i] = Color::Green as u8;
            continue;
        }
        if bc.contains(&ac[i]) {
            mask[i] = Color::Yellow as u8;
        }
    }
}

fn calc_entropy_for_word(q: &String, word_chars: &Vec<Vec<char>>) -> f64 {
    let qc: Vec<char> = q.chars().collect();

    let mut mask_map = HashMap::new();

    for wc in word_chars {
        let mut mask = [Color::Grey as u8, Color::Grey as u8, Color::Grey as u8, Color::Grey as u8, Color::Grey as u8];
        make_mask(&qc, &wc, &mut mask);
        let count = mask_map.entry(mask).or_insert(0);
        *count += 1;
    }

    let mut entropy = 0.0;

    for (_, count) in mask_map.iter() {
        let p = *count as f64 / mask_map.len() as f64;
        entropy += p * p.log2();
    }

    -entropy
}

fn get_best_word(words: &Vec<String>, legal_words: &Vec<String>) -> (String, f64) {
    let mut entropy = Vec::new();

    let word_chars: Vec<Vec<char>> = words.iter().map(|x| x.chars().collect()).collect();

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
        let mut mask = [Color::Grey as u8, Color::Grey as u8, Color::Grey as u8, Color::Grey as u8, Color::Grey as u8];
        make_mask(
            &"rebut".to_string().chars().collect(),
            &"rebut".to_string().chars().collect(),
            &mut mask,
        );

        assert_eq!(mask, [Color::Green as u8, Color::Green as u8, Color::Green as u8, Color::Green as u8, Color::Green as u8]);

        let mut mask2 = [Color::Grey as u8, Color::Grey as u8, Color::Grey as u8, Color::Grey as u8, Color::Grey as u8];
        make_mask(
            &"rebut".to_string().chars().collect(),
            &"butch".to_string().chars().collect(),
            &mut mask2,
        );

        assert_eq!(mask2, [Color::Grey as u8, Color::Grey as u8, Color::Yellow as u8, Color::Yellow as u8, Color::Yellow as u8]);
    }
}

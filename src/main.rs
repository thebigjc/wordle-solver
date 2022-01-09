use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn load_words() -> io::Result<Vec<String>> {
    let file = File::open("words2.txt")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn make_mask(ac: &Vec<char>, bc: &Vec<char>, mask: &mut [char; 5]) {
    for i in 0..mask.len() {
        if ac[i] == bc[i] {
            mask[i] = 'g';
            continue;
        }
        if bc.contains(&ac[i]) {
            mask[i] = 'y';
        }
    }
}

fn calc_entropy_for_word(q: &String, word_chars: &Vec<Vec<char>>) -> f64 {
    let qc: Vec<char> = q.chars().collect();

    let mut mask_map = HashMap::new();

    for wc in word_chars {
        let mut mask = ['r', 'r', 'r', 'r', 'r'];
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

fn get_best_word(words: &Vec<String>) -> (&String, f64) {
    let mut entropy = HashMap::new();

    let word_chars: Vec<Vec<char>> = words.iter().map(|x| x.chars().collect()).collect();

    for q in words {
        entropy.insert(q, calc_entropy_for_word(q, &word_chars));
        println!("{} has entropy {}", q, entropy.get(q).unwrap());
    }

    let mut sorted_entropy = entropy.iter().collect::<Vec<(&&String, &f64)>>();

    sorted_entropy.sort_by(|x, y| x.1.partial_cmp(y.1).unwrap());

    for (s, f) in &sorted_entropy {
        println!("{}: {}", s, f);
    }

    let (s, f) = sorted_entropy.last().unwrap();
    (*s, **f)
}

fn main() -> io::Result<()> {
    let words = load_words()?;

    println!("{}", words.len());
    let (first_guess, entrop) = get_best_word(&words);
    println!("let's guess: {} which has entropy: {}", first_guess, entrop);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_match() {
        let mut mask = ['r', 'r', 'r', 'r', 'r'];
        make_mask(
            &"rebut".to_string().chars().collect(),
            &"rebut".to_string().chars().collect(),
            &mut mask,
        );

        assert_eq!(mask, ['g', 'g', 'g', 'g', 'g']);

        let mut mask2 = ['r', 'r', 'r', 'r', 'r'];
        make_mask(
            &"rebut".to_string().chars().collect(),
            &"butch".to_string().chars().collect(),
            &mut mask2,
        );

        assert_eq!(mask2, ['r', 'r', 'y', 'y', 'y']);
    }
}

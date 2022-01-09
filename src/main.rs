use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn load_words() -> io::Result<Vec<String>> {
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[derive(Debug)]
enum MatchType {
    Green,
    Yellow,
    Grey,
}

fn match_iter() -> std::slice::Iter<'static, MatchType> {
    [MatchType::Green, MatchType::Yellow, MatchType::Grey].iter()
}

fn is_match(ac: &Vec<char>, bc: &Vec<char>, mask: &[&MatchType; 5]) -> bool {
    for i in 0..mask.len() {
        let b = match mask[i] {
            MatchType::Green => ac[i] == bc[i],
            MatchType::Yellow => bc.contains(&ac[i]),
            MatchType::Grey => !bc.contains(&ac[i]),
        };
        if !b {
            return false;
        }
    }

    return true;
}

fn make_mask(ac: &Vec<char>, bc: &Vec<char>, mut mask : [MatchType; 5]) {
    for i in 0..mask.len() {
        if ac[i] == bc[i] {
            mask[i] = MatchType::Green;
            continue;
        }
        if bc.contains(&ac[i]) {
            mask[i] = MatchType::Yellow;
        }
    }
}

fn calc_entropy_for_word(
    q: &String,
    word_chars: &Vec<Vec<char>>,
    match_mask: &Vec<[&MatchType; 5]>,
) -> f64 {
    let mut entropy = 0.0;

    let qc: Vec<char> = q.chars().collect();

//    let mut mask_map = HashMap.new();

    for m in match_mask {
        let mut count = 0;
        for wc in word_chars {
            if is_match(&qc, &wc, m) {
                count += 1;
            }
        }
        if count > 0 {
            let p = count as f64 / match_mask.len() as f64;
            entropy += p * p.log2();
        }
    }

    -entropy
}

fn get_best_word(words: &Vec<String>) -> (&String, f64) {
    let mut entropy = HashMap::new();

    let mut match_mask: Vec<[&MatchType; 5]> = Vec::new();
    for a in match_iter() {
        for b in match_iter() {
            for c in match_iter() {
                for d in match_iter() {
                    for e in match_iter() {
                        let mask = [a, b, c, d, e];
                        match_mask.push(mask);
                    }
                }
            }
        }
    }

    let word_chars: Vec<Vec<char>> = words.iter().map(|x| x.chars().collect()).collect();

    for q in words {
        entropy.insert(q, calc_entropy_for_word(q, &word_chars, &match_mask));
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
        assert_eq!(
            is_match(
                &"rebut".to_string().chars().collect(),
                &"rebut".to_string().chars().collect(),
                &[
                    &MatchType::Green,
                    &MatchType::Green,
                    &MatchType::Green,
                    &MatchType::Green,
                    &MatchType::Green
                ]
            ),
            true
        );
        assert_eq!(
            is_match(
                &"rebut".to_string().chars().collect(),
                &"butch".to_string().chars().collect(),
                &[
                    &MatchType::Grey,
                    &MatchType::Grey,
                    &MatchType::Yellow,
                    &MatchType::Green,
                    &MatchType::Yellow
                ]
            ),
            false
        );
    }
}

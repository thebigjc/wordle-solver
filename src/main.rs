use std::fs::File;
use std::io::{self, prelude::*, BufReader};



fn load_words(f: &str) -> io::Result<Vec<String>> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/*fn make_mask(ac: &Vec<char>, bc: &Vec<char>, mask: &mut Mask) {
    for i in 0..mask.len() {
        if ac[i] == bc[i] {
            mask[i] = Color::Green as u8;
            continue;
        }
        if bc.contains(&ac[i]) {
            mask[i] = Color::Yellow as u8;
        }
    }
}*/

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
enum Color {
    Grey,
    Yellow,
    Green
}

#[derive(PartialEq,Eq,Hash)]
struct Mask (Color, Color, Color, Color, Color);

impl Mask {
    fn index(&self) -> usize {
        (self.0 as u8 + (self.1 as u8) * 3 + (self.2 as u8 * 3 * 3) + (self.3 as u8 * 3 * 3 * 3) + (self.4 as u8 * 3 * 3 * 3 * 3)) as usize 
    }

    fn make(ac: &Vec<char>, bc: &Vec<char>) -> Mask {
        let (a, b, c, d, e);

        if ac[0] == bc[0] {
            a = Color::Green;
        } else if bc.contains(&ac[0]) {
            a = Color::Yellow;
        } else {
            a = Color::Grey;
        }

        if ac[1] == bc[1] {
            b = Color::Green;
        } else if bc.contains(&ac[1]) {
            b = Color::Yellow;
        } else {
            b = Color::Grey;
        }

        if ac[2] == bc[2] {
            c = Color::Green;
        } else if bc.contains(&ac[2]) {
            c = Color::Yellow;
        } else {
            c = Color::Grey;
        }

        if ac[3] == bc[3] {
            d = Color::Green;
        } else if bc.contains(&ac[3]) {
            d = Color::Yellow;
        } else {
            d = Color::Grey;
        }

        if ac[4] == bc[4] {
            e = Color::Green;
        } else if bc.contains(&ac[4]) {
            e = Color::Yellow;
        } else {
            e = Color::Grey;
        }

        Mask (a, b, c, d, e)
    }
}

const MASK_SIZE : usize = 3 * 3 * 3 * 3 * 3;

fn calc_entropy_for_word(q: &String, word_chars: &Vec<Vec<char>>) -> f64 {
    let qc: Vec<char> = q.chars().collect();

    let mut mask_map : [u8; MASK_SIZE] = [0; MASK_SIZE];

    for wc in word_chars {
        let mask = Mask::make(&qc, &wc);
        mask_map[mask.index()]+= 1;
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

//! Green and Yellow, Complete Version

const NUM_DIGITS: usize = 4;

fn calc_green_and_yellow(
    guess: &[u8; NUM_DIGITS],
    secret: &[u8; NUM_DIGITS],
) -> [char; NUM_DIGITS] {
    let mut result = ['⬜'; NUM_DIGITS];
    let mut secret_used = [false; NUM_DIGITS];

    for i in 0..NUM_DIGITS {
        if guess[i] == secret[i] {
            // that's a match
            result[i] = '🟩';
            // don't match this secret digit again
            secret_used[i] = true;
        }
    }

    for index_g in 0..NUM_DIGITS {
        // only process guess digits that weren't a perfect match
        if result[index_g] != '🟩' {
            for index_s in 0..NUM_DIGITS {
                // does the guess digit match that secret digit (and is that secret digit unused so far?)
                if (guess[index_g] == secret[index_s]) && !secret_used[index_s] {
                    // this is a correct digit but in the wrong place
                    result[index_g] = '🟨';
                    // don't match this secret digit again
                    secret_used[index_s] = true;
                    // move to next guess digit now
                    break;
                }
            }
        }
    }

    result
}

fn main() {
    let stdin = std::io::stdin();

    println!("New game!");

    let mut secret = [0u8; NUM_DIGITS];
    for digit in secret.iter_mut() {
        *digit = rand::random_range(1..=9);
    }

    'guess_loop: loop {
        let mut line = String::new();
        println!("Enter guess:");
        stdin.read_line(&mut line).unwrap();
        let mut guess = [0u8; NUM_DIGITS];
        let mut idx = 0;
        for piece in line.trim().split(' ') {
            let Ok(digit) = piece.parse::<u8>() else {
                println!("{:?} wasn't a number", piece);
                continue 'guess_loop;
            };
            if digit < 1 || digit > 9 {
                println!("{} is out of range", digit);
                continue 'guess_loop;
            }
            if let Some(slot) = guess.get_mut(idx) {
                *slot = digit;
            } else {
                println!("Too many numbers, I only want 4!");
                continue 'guess_loop;
            }
            idx += 1;
        }
        if idx < guess.len() {
            println!("Not enough numbers, I want {}", guess.len());
            continue 'guess_loop;
        }

        println!("Your guess is {:?}", guess);

        let score = calc_green_and_yellow(&guess, &secret);

        let nice_string: String = score.iter().collect();
        println!("That gives: {}", nice_string);

        if guess == secret {
            println!("Well done!!");
            break;
        }
    }
}

#[test]
fn all_wrong() {
    assert_eq!(
        calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
        ['⬜', '⬜', '⬜', '⬜']
    );
}

#[test]
fn all_green() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        ['🟩', '🟩', '🟩', '🟩']
    );
}

#[test]
fn one_wrong() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        ['🟩', '🟩', '🟩', '⬜']
    );
}

#[test]
fn all_yellow() {
    assert_eq!(
        calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        ['🟨', '🟨', '🟨', '🟨']
    );
}

#[test]
fn one_wrong_but_duplicate() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        ['🟩', '🟩', '🟩', '⬜']
    );
}

#[test]
fn one_right_others_duplicate() {
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        ['🟩', '⬜', '⬜', '⬜']
    );
}

#[test]
fn two_right_two_swapped() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        ['🟨', '🟩', '🟩', '🟨']
    );
}

#[test]
fn two_wrong_two_swapped() {
    assert_eq!(
        calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        ['🟨', '⬜', '⬜', '🟨']
    );
}

#[test]
fn a_bit_of_everything() {
    assert_eq!(
        calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
        ['🟩', '⬜', '🟨', '🟨']
    );
}

#[test]
fn two_in_guess_one_in_secret() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 3], &[3, 9, 9, 9]),
        ['⬜', '⬜', '🟨', '⬜']
    );
}

#[test]
fn four_in_guess_one_in_secret() {
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 1], &[4, 3, 1, 2]),
        ['⬜', '⬜', '🟩', '⬜']
    );
}

#[test]
fn one_in_guess_two_in_secret() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
        ['⬜', '⬜', '🟨', '⬜']
    );
}

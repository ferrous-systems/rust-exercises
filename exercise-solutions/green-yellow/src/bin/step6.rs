//! Green and Yellow, Step 6

use rand::Rng;

#[allow(unused)]
fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String {
    let mut result = ["⬜"; 4];
    let mut guess = *guess;
    let mut secret = *secret;

    for i in 0..guess.len() {
        if guess[i] == secret[i] {
            result[i] = "🟩";
            secret[i] = 0;
            guess[i] = 0;
        }
    }

    for i in 0..guess.len() {
        for j in 0..secret.len() {
            if guess[i] == secret[j] && secret[j] != 0 && guess[i] != 0 {
                result[i] = "🟨";
            }
        }
    }

    result.join("")
}

fn main() {
    let mut rng = rand::thread_rng();
    let stdin = std::io::stdin();

    let mut secret = [0u8; 4];
    for digit in secret.iter_mut() {
        *digit = rng.gen_range(1..=9);
    }

    println!("New game (secret is {:?}!", secret);

    loop {
        let mut line = String::new();
        println!("Enter guess:");
        stdin.read_line(&mut line).unwrap();
        let mut guess = [0u8; 4];
        let mut idx = 0;
        for piece in line.trim().split(' ') {
            let Ok(digit) = piece.parse::<u8>() else {
                println!("{:?} wasn't a number", piece);
                continue;
            };
            if digit < 1 || digit > 9 {
                println!("{} is out of range", digit);
                continue;
            }
            if idx >= guess.len() {
                println!("Too many numbers, I only want {}", guess.len());
                continue;
            }
            guess[idx] = digit;
            idx += 1;
        }
        if idx < guess.len() {
            println!("Not enough numbers, I want {}", guess.len());
            continue;
        }
        println!("Your guess is {:?}", guess);
        if guess == secret {
            println!("Well done!!");
            break;
        }
    }
}

#[test]
fn all_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
        "⬜⬜⬜⬜"
    );
}

#[test]
fn all_green() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩🟩🟩🟩"
    );
}

#[test]
fn one_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn all_yellow() {
    assert_eq!(
        &calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨🟨🟨🟨"
    );
}

#[test]
fn one_wrong_but_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn one_right_others_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        "🟩⬜⬜⬜"
    );
}

#[test]
fn two_right_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨🟩🟩🟨"
    );
}

#[test]
fn two_wrong_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨⬜⬜🟨"
    );
}

#[test]
fn a_bit_of_everything() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
        "🟩⬜🟨🟨"
    );
}

#[test]
fn two_in_guess_one_in_secret() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 3], &[3, 9, 9, 9]),
        "⬜⬜🟨⬜"
    );
}

#[test]
fn two_in_secret_one_in_guess() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
        "⬜⬜🟨⬜"
    );
}

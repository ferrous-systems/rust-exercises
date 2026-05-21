//! Green and Yellow, Step 4

fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String {
    let mut result = ["⬜"; 4];
    let mut secret_handled = [false; 4];

    for i in 0..guess.len() {
        if guess[i] == secret[i] {
            // that's a match
            result[i] = "🟩";
            // don't match this secret digit again
            secret_handled[i] = true;
        }
    }

    'guess: for g_idx in 0..guess.len() {
        // only process guess digits we haven't already dealt with
        if result[g_idx] == "🟩" {
            continue;
        }
        for s_idx in 0..secret.len() {
            // only process secret digits we haven't already dealt with
            if secret_handled[s_idx] {
                continue;
            }
            if guess[g_idx] == secret[s_idx] {
                // put a yellow block in for this guess
                result[g_idx] = "🟨";
                // never match this secret digit again
                secret_handled[s_idx] = true;
                // stop comparing this guessed digit to any other secret digits
                continue 'guess;
            }
        }
    }

    result.join("")
}

fn main() {
    println!("{}", calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 4, 4]));
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
fn four_in_guess_one_in_secret() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 1, 1, 1], &[4, 3, 1, 2]),
        "⬜⬜🟩⬜"
    );
}

#[test]
fn two_in_secret_one_in_guess() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[3, 3, 9, 9]),
        "⬜⬜🟨⬜"
    );
}

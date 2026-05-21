//! Green and Yellow, Step 3

fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String {
    let mut result = ["⬜"; 4];

    for i in 0..guess.len() {
        if guess[i] == secret[i] {
            result[i] = "🟩";
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

//! Green and Yellow, Step 3

const NUM_DIGITS: usize = 4;

fn calc_green_and_yellow(
    guess: &[u8; NUM_DIGITS],
    secret: &[u8; NUM_DIGITS],
) -> [char; NUM_DIGITS] {
    let mut result = ['⬜'; NUM_DIGITS];

    for i in 0..NUM_DIGITS {
        if guess[i] == secret[i] {
            // that's a match
            result[i] = '🟩';
        }
    }

    result
}

fn main() {
    println!("{:?}", calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 4, 4]));
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

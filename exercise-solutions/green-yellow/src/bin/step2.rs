//! Green and Yellow, Step 2

const NUM_DIGITS: usize = 4;

fn calc_green_and_yellow(
    _guess: &[u8; NUM_DIGITS],
    _secret: &[u8; NUM_DIGITS],
) -> [char; NUM_DIGITS] {
    let result = ['⬜'; NUM_DIGITS];

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

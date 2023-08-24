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

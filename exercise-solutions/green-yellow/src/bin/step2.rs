//! Green and Yellow, Step 2

fn calc_green_and_yellow(_guess: &[u8; 4], _secret: &[u8; 4]) -> String {
    let result = ["⬜"; 4];

    result.join("")
}

fn main() {
    println!("{}", calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 4, 4]));
}

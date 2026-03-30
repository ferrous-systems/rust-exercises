fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(i: u32) -> String {
    if i.is_multiple_of(3) && i.is_multiple_of(5) {
        format!("FizzBuzz")
    } else if i.is_multiple_of(3) {
        format!("Fizz")
    } else if i.is_multiple_of(5) {
        format!("Buzz")
    } else {
        format!("{}", i)
    }
}

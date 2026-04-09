fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(i: u32) -> String { 
    match (i.is_multiple_of(3), i.is_multiple_of(5)) {
        (true, true) => format!("FizzBuzz"),
        (true, false) => format!("Fizz"),
        (false, true) => format!("Buzz"),
        (false, false) => format!("{}", i),
    }
}

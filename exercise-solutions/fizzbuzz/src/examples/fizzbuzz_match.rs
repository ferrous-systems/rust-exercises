fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(i: u32) -> String {

    let remainders = (i%3, i%5);
    
    match remainders {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        (_, _) => format!("{}", i),
    }
}

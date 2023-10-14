use std::io;

fn fibonacci_nth(value: u32) -> u32 {
    if value <= 0 {
        return 0;
    } else if value == 1 {
        return 1;
    }
    fibonacci_nth(value - 1) + fibonacci_nth(value - 2)
}

fn main() {
    // ask the user for which number in the sequence they wish to find
    println!("Hey there, I can find your number in the sequence, which one do you want me to find?");
    let mut user_value = String::new();
    io::stdin()
        .read_line(&mut user_value)
        .expect("Please enter a value");
    let user_value: u32 = user_value.trim().parse().expect("Invalid input");
    // once its read process this through a fibonacci function and return it
    println!("Nth number in the Fibonacci Sequence is: {}", fibonacci_nth(user_value));
}

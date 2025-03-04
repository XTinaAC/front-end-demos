// If a type you wanna use isn't included in the [prelude] (which Rust brings into the scope of every program),
// you have to bring it into the current scope explicitly with a [use] statement.
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // The [rand::thread_rng] method in the [rand::Rng] trait gives us a random number generator local to the current thread of
    // execution and is seeded by the operating system. 
    let secret_num = rand::rng().random_range(1..=10);

    loop {
    // println! is a [macro] that prints a string to the screen:
    println!("Please input your guess (positive integers between 1 and 10)! press 0 to quit.");
    
    // To make a variable mutable, [mut] is needed before the variable name;
    // [String] is a type that is a growable, UTF-8 encoded bit of text, provided by the standard library;
    // The [::] syntax indicates that [new] is an [associated function] of the String type (a concept similar to 
    // [static method] in some other programming languages).
    let mut guess = String::new();

    // The [stdin] function returns an instance of [std::io::Stdin] which represents the standard input handle;
    // The [&] indicates that the argument is a [reference];
    // [read_line] not only puts whatever the user enters into the string we pass to it, but it also returns a 
    // [Result] value, which is an [enumeration] that includes multiple [states/variants]: (1)Ok; (2)Err; and that have methods defined on them:
    
    // Here [expect] is used to:
    // (1) crash the program if [read_line] returns an [Err], and display the message that we passed as an argument to [expect];
    // (2) return the number of bytes of in the user's input if [read_line] returns an [Ok].
    let number_of_bytes = io::stdin().read_line(&mut guess).expect("Failed to read a line");

    println!("You guessed: {}(Number of bytes: {})", guess, number_of_bytes);


    // Ignore a non-number guess instead of crashing the program and proceed to the next loop;
    // Quit if the user typed in '0'.
    // We must use [trim] to eliminate any whitespace at the beginning and end of a string in order to compare it to an u32 integer.
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => if num==0 { break } else { num },
        Err(_) => continue,
    };

    // A [match] expression is made up of [arms] which consists of a [pattern] to match against,
    // and the code that should be run if the value given to [match] fits that arm's pattern.
    // Quit if there's a match.
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("Bingo");
            break;
        }
    }
    }
}

use std::fs::File;
use std::io;

pub struct Guess {
    value: i32
}
/*
 * Code outside the module must use the [Guess::new] function to create an instance of [Guess],
 * and use the [Guess::value] getter to get some data from private fields and return it.
 */
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 10 {
            panic! ("guess value must be an integer between 1 and 10, got {}", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() { 
    println!("Please input your guess(an integer between 1 and 10");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read a line");
    /*
     *  The [unwrap] method is a shortcut like the [match] expression:
     *      if the [Result] is the [Ok] variant, [unwrap] will return the value inside the [Ok];
     *      if the [Result] is the [Err] variant, [unwrap] will call the [panic!] macro for us.
     *  The [expect] method works the same way as [unwrap]; besides,
     *      it will use the error message we provide as the parameter to pass to [panic!],
     *      instead of the default [panic!] message that [unwrap] uses.
     */
    let guess = Guess::new(guess.trim().parse().unwrap()).value();

    println!("You guessed: {}", guess);


    let v = vec![1, 2, 3];
    v[99];
    /*
     * thread 'main' panicked at src/main.rs:42:6:
        index out of bounds: the len is 3 but the index is 99
    -> run with `RUST_BACKTRACE=1` environment variable to display a backtrace:
        RUST_BACKTRACE=1 cargo run
    -> stack backtrace:
        0: rust_begin_unwind
        1: core::panicking::panic_fmt
        2: core::panicking::panic_bounds_check
        3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rust/src/library/core/src/slice/index.rs:274:10
        4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rust/src/library/core/src/slice/index.rs:16:9
        5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rust/src/library/alloc/src/vec/mod.rs:3346:9
        6: rust_lang::main
             at ./main.rs:42:6
        7: core::ops::function::FnOnce::call_once
             at /rust/src/library/core/src/ops/function.rs:250:5
        note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    */
}

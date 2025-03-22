use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
   
    let v = vec![1, 2, 3];
    v[99];
    /*
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
             at ./main.rs:6:6
        7: core::ops::function::FnOnce::call_once
             at /rust/src/library/core/src/ops/function.rs:250:5
        note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    */
}

/// Expected Behavior
/// - Yew Trunk should execute `./src/main.rs::main()`
/// - Yew Trunk should ignore  `./src/bin/timer.rs::main()`
///
/// What actually happens:
/// - Yew Trunk compiles all `main()` functions to WASM and runs on browser startup

fn main() {
    // BUG: This function should be CLI only and not panic in Browser WASM Yew Trunk
    time_summation(1_000_000);
}

fn time_summation(count: u64) {
    // BUG: std::time::Instant works on linux, but panics in browser WASM
    let time_start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for i in 0..count {
        sum += i;
    }
    let time_taken = time_start.elapsed();
    let message = format!("timer.rs: sum(0..{}) = {} in ({:.2?})", count, sum, time_taken);
    println!("{}", message);
}
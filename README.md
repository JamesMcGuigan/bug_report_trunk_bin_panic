# Bug Report: Trunk Bin Panic

Working Baseline Yew Trunk website
```
git checkout 556909a3bc3ec2175ce812fc9190723ed4bd5403
trunk serve
```

Adding an additional CLI binary [./src/bin/timer.rs](./src/bin/timer.rs) 
works locally:
```
$ git checkout -f
$ cargo run --bin timer -q
timer.rs: sum(0..1000000) = 499999500000 in (17.40ms)

$ cargo build --release
$ ./target/release/timer 
timer.rs: sum(0..1000000) = 499999500000 in (41.00ns)
```

But causes the Trunk Browser WASM to panic
- std::time::Instant::now() is not supported in WASM
- `trunk serve` causes intermittent panic (depending on build order)
- `trunk build` always panics
- why is: `timer::main() -> time_summation()` being called from WASM ??? 
```
$ git checkout -f
trunk serve  # intermittent panic
trunk build  # production panic
http-server ./dist/ -p 8000  
```
Chrome:
```
trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xc2e6 Uncaught (in promise) RuntimeError: unreachable
    at __rust_start_panic (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xc2e6)
    at rust_panic (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xbc5e)
    at std::panicking::rust_panic_with_hook::hfb395eb735d1b98d (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0x8064)
    at std::panicking::begin_panic_handler::{{closure}}::h197c093a465fbcb7 (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0x9097)
    at std::sys_common::backtrace::__rust_end_short_backtrace::habb14bc6f7baace9 (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xbfaa)
    at rust_begin_unwind (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xb6fe)
    at core::panicking::panic_fmt::ha22af367754efe7c (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xb838)
    at std::time::Instant::now::h9b2f55ef07b8b699 (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xb776)
    at timer::time_summation::h56c34a274427d2f4 (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0x2ef1)
    at timer::main::h2b998d23f6e5bd5b (trunk_bin_panic-df2f221d8ccc154a_bg.wasm:0xc0b1)
```

Expected Behavior
- Yew Trunk should execute `./src/main.rs::main()`
- Yew Trunk should ignore  `./src/bin/timer.rs::main()`

What actually happens:
- Yew Trunk compiles all `main()` functions to WASM and runs on browser startup
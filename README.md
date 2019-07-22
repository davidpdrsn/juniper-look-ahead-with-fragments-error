This repo is for reproducing a bug in [Juniper][].

Run `cargo run` to reproduce the error:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/juniper-look-ahead-with-fragments-error`
thread 'main' panicked at 'lookahead to find the field', src/libcore/option.rs:1036:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

Tested on `rustc 1.36.0 (a53f9df32 2019-07-03)`

[Juniper]: https://github.com/graphql-rust/juniper

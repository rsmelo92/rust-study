# Rust find word
Hello-word Rust POC

This finds a word in a given file

## Getting started

1 - Set up `Rust` environment in https://www.rust-lang.org/pt-BR/learn/get-started

2 - Clone the project

3 - Run `cargo run -- <word-to-search> <file-path-to-search-in>`

### Example:
```rust
cargo run -- coder ./english-words.txt
```

The output should be
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/grrs coders english-words.txt`
coders
decoders
encoders
vocoders
```

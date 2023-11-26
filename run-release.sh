#!/bin/bash

# run build first: cargo build --release
rustc main.rs --extern day1=target/release/libday1.rlib
time ./main


# References
# https://doc.rust-lang.org/stable/rust-by-example/crates/lib.html
# https://stackoverflow.com/questions/50731453/how-to-statically-link-to-an-existing-rlib


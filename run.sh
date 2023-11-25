#!/bin/bash

# run cargo build first
rustc main.rs --extern day1=target/debug/libday1.rlib
time ./main


# References
# https://doc.rust-lang.org/stable/rust-by-example/crates/lib.html
# https://stackoverflow.com/questions/50731453/how-to-statically-link-to-an-existing-rlib


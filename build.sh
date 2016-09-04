#!/bin/sh
clang -c -fmodules -Wall -Wextra objc/sample.m -o libsample.o
ar rcs libsample.a libsample.o
cargo rustc -- -L.
./target/debug/objc_rust

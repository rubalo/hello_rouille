# Hello rouille

Rouille is the french world for Rust. Hello_rouille is a contract of hello_world and rouille. Basically,
I'm learning rust. 

From the following book : https://doc.rust-lang.org/stable/book/

And this is a placeholder for the code.  Nothing fancy here.

# 0. Reference

    Preludes : Librairies shipped with rust -> https://doc.rust-lang.org/stable/std/prelude/index.html

# 1. Getting started

Rustup: 
    Command line tool to manage Rust versions.
    Uninstall rustup :  `rustup self uninstall`

rustfmt: 
    Format code

Cargo:
    Cargo is Rust’s build system and package manager.
    Create project: `cargo new hello_cargo`
    Build: `cargo build`
        --release for longer compile but faster code
    Run: `cargo run` (Binary in target folder)
    Check: `cargo check` Doesn't produce an exe
    Update crates : `cargo update`


`println!` is a macro and `println` is a function.

# 2. Programming gessing game

    An associated function is a function that’s implemented on a type


NEXT : https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number
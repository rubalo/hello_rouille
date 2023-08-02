# Hello rouille

Rouille is the french world for Rust. Hello_rouille is a contract of hello_world and rouille. Basically,
I'm learning rust. 

From the following book : https://doc.rust-lang.org/stable/book/

And this is a placeholder for the code.  Nothing fancy here.

# 0. Reminders

    Preludes : Librairies shipped with rust -> https://doc.rust-lang.org/stable/std/prelude/index.html

    rust-analyzer to work with subfolders in vs-code:
        add in .vscode/settings.json  a link to the Cargo.toml, for example:
        "rust-analyzer.linkedProjects": ["projects/guessing_game/Cargo.toml"]

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
    Local doc: `cargo doc --open`


`println!` is a macro and `println` is a function.

# 2. Programming gessing game

    An associated function is a function that’s implemented on a type

# 3. Common Programming Concepts

# 4. Understanding Ownership

# 5. Using Structs to Structure Related Data

# 6. Enums and Pattern Matching

# 7. Managing Growing Projects with Packages, Crates, and Modules

# 8. Common collections

    vector, strings ad hashmaps

# 9. Error Handling

# 10. Generic types, Traits and Lifetimes

# 11. Wrting automated Tests

# 12. An I/O Project: Building a Command Line Program

NEXT : https://doc.rust-lang.org/stable/book/ch12-05-working-with-environment-variables.html
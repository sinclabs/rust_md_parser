# Markdown to HTML compiler based on Rust

*Note: This project is under heavy development and at very early stages.*

## **Motivation:**
This project is inteded to give a super optimized, blazing fast Markdown to HTML complier using the near native speeds of WebAssembly. This is achieved through compiling Rust to WebAssembly.

## Markdown Syntax Support:
Currently only the basic syntax is under construction [Ref](https://www.markdownguide.org/cheat-sheet/). In future, support for different flavours will be added.

## Setup:
- Make sure you have Rust and Cargo installed. If not, do so using the instructions provided [here](https://www.rust-lang.org/learn/get-started).
- Clone the repo:
    ```bash
    git clone https://github.com/sinclabs/rust_md_parser.git
- From the root directory of the project, run:
    ```bash
    cargo build && cargo run

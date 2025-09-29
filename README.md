# Markov Chain in Rust

Some Markov Chain implementations in Rust

## Features

***Gambler's ruin***

- A simulation where you have X amount of money, each game will have a bet of Y amount of money. You have 50% of winning and 50% of losing it. The simulation ends when either you doubled your money, or you lost them all.

***Sentence Generator***

- A sentence generator that take a string input and spit out whatever the word it random to based on the previous word.

## Build from source

>Make sure that you have Rust installed!

1. Clone the repo

    ```bash
    $ git clone https://github.com/thqnhz/Markov-Chain-Rust markov-chain-rust
    $ cd markov-chain-rust
    ```

2. Build the project

    ```bash
    $ cargo build
    ```

3. Run the executable

    ```bash
    $ cargo run
    ```

    >Or run the executable file inside the `target/debug/` directory.

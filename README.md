# Guessing Game in Rust

A command-line number-guessing game written in Rust. Each run selects a
random secret number from **1 through 100**, then repeatedly prompts the
player until the number is guessed or the player quits.

## Features

- Generates a new random number for every run.
- Uses the inclusive range `1..=100` for the secret number.
- Reports whether each numeric guess is too low or too high.
- Accepts `q` to quit the game.
- Handles non-numeric input without terminating the program.
- Ends immediately after a correct guess.

## Requirements

- Rust and Cargo with support for the Rust 2024 edition.

The project has been developed with the standard Rust toolchain. Verify that
the tools are installed with:

```text
rustc --version
cargo --version
```

## Getting started

Clone or download the project, then change into its directory:

```bash
cd guessing_game_in_rust
```

Build the project:

```bash
cargo build
```

Run the game:

```bash
cargo run
```

For an optimized release build:

```bash
cargo build --release
```

The release executable is written to `target/release/guessing_game_in_rust`.

## How to play

1. Start the game with `cargo run`.
2. Enter a whole-number guess when prompted.
3. Use the feedback to adjust the next guess:
   - `Guess is too low` means the secret number is greater than the guess.
   - `Guess is too high` means the secret number is less than the guess.
4. Continue until the game prints the success message.
5. Enter `q` at any prompt to quit.

Example session:

```text
Guess the number!
To Quit the game, enter 'q'
Please enter your guess (between 1 to 100) :
50
You guessed: 50
Guess is too low
Please enter your guess (between 1 to 100) :
75
You guessed: 75
Guess is too high
Please enter your guess (between 1 to 100) :
63
You guessed: 63
Congrats! You have guessed the number
```

The secret number is random, so the exact guesses and feedback will differ
between runs.

## Input behavior

- Input is read one line at a time from standard input.
- `q` quits after trimming surrounding whitespace, so `q`, ` q`, and `q `
  are treated the same.
- Numeric input is parsed as a signed 32-bit integer.
- Non-numeric input produces `Enter a valid number between 1 to 100!` and the
  game continues.
- The secret number is restricted to 1–100. The current implementation does
  not explicitly reject numeric guesses outside that range; such guesses are
  compared with the secret number normally.
- End-of-input or another standard-input read failure terminates the program
  through the `read_line` error expectation.

## Project structure

```text
.
├── Cargo.toml       # Package metadata and dependencies
├── Cargo.lock       # Locked dependency versions
├── README.md        # Project documentation
└── src
    └── main.rs      # CLI entry point and game loop
```

Build output and other generated files are placed in `target/`, which is
ignored by Git.

## Implementation overview

The application is a single binary crate:

1. `rand::rng()` creates the random-number generator.
2. `random_range(1..=100)` selects the secret number.
3. A loop reads guesses from standard input.
4. The trimmed input is checked for the quit command.
5. Remaining input is parsed as `i32`.
6. `std::cmp::Ordering` compares valid guesses with the secret number.
7. The loop exits on `q` or a correct guess.

## Dependencies

The only direct dependency is:

- [`rand`](https://crates.io/crates/rand) `0.10.3` — random-number generation.

Cargo resolves transitive dependencies using `Cargo.lock`.

## Development

Format the source code:

```bash
cargo fmt
```

Check compilation without producing a final executable:

```bash
cargo check
```

Run the test suite:

```bash
cargo test
```

The project currently contains no automated tests; `cargo test` still verifies
that the test profile compiles successfully. Gameplay is currently validated
interactively through the command line.

## Troubleshooting

### Cargo cannot download dependencies

The first build requires access to the configured Cargo registry. Check your
network connection and run `cargo build` again.

### The game does not accept my input

Enter a value followed by **Enter**. Input is line-oriented rather than
character-oriented.

### I entered text and the game continued

This is expected for non-numeric input. The game prints a validation message
and prompts for another guess.

## License

No license file or license declaration is currently included in this project.
Add one before distributing the project if a specific reuse license is
required.
# rust-simple-project

This repository contains a simple Rust command-line application called **quote_of_the_day**.

## Project Overview

**quote_of_the_day** is a CLI tool that prints a random inspirational quote each time you run it. It demonstrates basic Rust concepts such as:
- Using external crates (like `rand` for randomness)
- Organizing code with functions and modules
- Writing and running unit tests
- Setting up CI/CD with GitHub Actions

## How to Use

1. Navigate to the `quote_of_the_day` directory:
   ```sh
   cd quote_of_the_day
   ```
2. Build and run the app:
   ```sh
   cargo run
   ```

## Running Tests

From the `quote_of_the_day` directory, run:
```sh
cargo test
```

## Continuous Integration

This project uses GitHub Actions to automatically build, test, and celebrate successful runs with a fun fact whenever you push or open a pull request.
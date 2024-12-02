# Rust 2024 Advent of Code

## Rust

To create a new day: `cargo xtask generate <#>` for day-01: `cargo xtask generate 1`

To run a part: `cargo r <day#> <part#>` for day-01 part1: `cargo r 1 1`
To test a part: `cargo t <day> <part>` for day-01 part1: `cargo t 1 part1`
To test both parts: `cargo t <day#>` for day-01: `cargo t 1`

To watch tests: `cargo watch -x 't <day#> <test-name>'` for day-01 part1: `cargo watch -x 't 1 part1'`

## Go

To create a new day: `just gen <#>` for day-01: `just gen 01`

You can set a default day, and part in `.env` and run with `just run`.

Or override them by providing them explicitly:

To run a part: `just run <day#> <part#>` for day-01 part1: `just run 01 1`
To test a part: `just test <day#> <part#>` for day-01 part1: `just test 01 1`
# Rust 2023 Advent of Code

To create a new day: `cargo xtask generate <#>` for day-01: `cargo xtask generate 1`

To run a part: `cargo r <day#> <part#>` for day-01 part1: `cargo r 1 1`
To test a part: `cargo t <day> <part>` for day-01 part1: `cargo t 1 part1`
To test both parts: `cargo t <day#>` for day-01: `cargo t 1`

To watch tests: `cargo watch -x 't <day#> <test-name>'` for day-01 part1: `cargo watch -x 't 1 part1'`

To run benches: `cargo xtask bench <day#> <part#>` for day-01 part1: `cargo xtask bench 1 1`

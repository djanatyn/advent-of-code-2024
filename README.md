advent-of-code-2024
===================

rust solutions to [2024 Advent of Code](https://adventofcode.com/2024/about)

## running a solution for a particular day

``` sh
$ cargo run --bin day01
```

## fetching problem text as markdown

this is for fetching problem descriptions and input when working on a solution.
you probably don't need to run this.

requires:
- `$SESSION_KEY` env variable set
- `rdrview`
- `pandoc`

``` sh
$ cargo run --bin fetch 2024/day/6
```

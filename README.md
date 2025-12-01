# Advent of code 2025

these are my [advent of code](https://adventofcode.com/2025) solutions for 2025 in Rust.

The syntax to run it is

```bash
cargo r -r -q --  01[:1|:2][,02[:1|:2] ...] 
```

if no part is provided both are run.

To make it quicker to create the performance table there is a justfile with a command `table`, it needs a list of days to create the table like

```bash
just table 01 02

```

## performance

| day | part | time |
| 01 | 1 | 126.682µs  |
| 01 | 2 | 102.447µs  |

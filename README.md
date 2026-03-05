# competitive-rust

Rust workspace for competitive programming (Codeforces, LeetCode, etc.). Each problem is its own crate.

## Structure

```
competitive-rust/
├── Cargo.toml          # workspace root
└── problems/
    └── fib_matrix/
        ├── Cargo.toml
        ├── input.txt   # put your problem input here
        └── src/
            └── main.rs
```

## Adding a new problem

```bash
cargo new problems/my_problem
```

Then add `input.txt` inside that directory and write your solution in `src/main.rs`.

## Running a problem

Solutions read from stdin. Redirect `input.txt` when running:

```bash
cargo run -p fib_matrix < problems/fib_matrix/input.txt
```

To also capture output:

```bash
cargo run -p fib_matrix < problems/fib_matrix/input.txt > output.txt
```

## Input format convention

```
n
<line 1>
<line 2>
...
<line n>
```

Output is formatted as:

```
tc #1: <answer>
tc #2: <answer>
...
```

## Notes

- Apply modulo `10^9 + 7` for large number problems (already done in the Fibonacci example)

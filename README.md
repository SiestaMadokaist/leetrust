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

Always run from the workspace root so the `input.txt` relative path resolves correctly.

```bash
cargo run -p fib_matrix
```

## Notes

- Each problem reads input from `problems/<name>/input.txt`
- Run commands from the workspace root
- Apply modulo `10^9 + 7` for large number problems (already done in the Fibonacci example)

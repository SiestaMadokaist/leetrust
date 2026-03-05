use std::fs;

// 2x2 matrix represented as [[a, b], [c, d]]
type Mat = [[u64; 2]; 2];

const MOD: u64 = 1_000_000_007;

fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    [
        [
            (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % MOD,
            (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % MOD,
        ],
        [
            (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % MOD,
            (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % MOD,
        ],
    ]
}

fn mat_pow(mut base: Mat, mut exp: u64) -> Mat {
    let mut result: Mat = [[1, 0], [0, 1]]; // identity matrix
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &base);
        }
        base = mat_mul(&base, &base);
        exp >>= 1;
    }
    result
}

// Returns F(n) where F(0)=0, F(1)=1
// [[1,1],[1,0]]^n = [[F(n+1), F(n)], [F(n), F(n-1)]]
fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let base: Mat = [[1, 1], [1, 0]];
    let result = mat_pow(base, n);
    result[0][1]
}

fn main() {
    let input = fs::read_to_string("problems/fib_matrix/input.txt")
        .expect("Could not read input.txt");

    let n: u64 = input.trim().parse().expect("Expected a non-negative integer");

    println!("{}", fib(n));
}

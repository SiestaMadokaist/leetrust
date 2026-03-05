fn solve(input: &String, mut start: usize) -> i32 {
    let mut changes = 0;
    let chars = vec!['0', '1'];
    for actual in input.chars() {
        start = 1 - start;
        let expected = chars[start];
        if expected != actual {
            changes = changes + 1;
        }
    }
    return changes;
}

fn answer(input: String) -> i32 {
    let s0 = solve(&input, 0);
    let s1 = solve(&input, 1);
    if s0 < s1 {
        return s0;
    } else {
        return s1;
    }
}

fn main() {
    let tc = "1111";
    let ans = answer(tc.to_string());
    println!("{}", ans);
}

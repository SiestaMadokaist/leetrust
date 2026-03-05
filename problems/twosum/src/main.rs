fn solve(vecs: Vec<i32>, target: i32) -> Vec<i32> {
    // let mut vecs = original.clone();
    // vecs.sort();
    let len = vecs.len() - 1;
    for i in 0..=len {
        let small = vecs[i];
        let next = i + 1;
        for j in next..=len {
            let big = vecs[j];
            let sum = small + big;
            // println!("({} {}): {} {} {}", i, j, small, big, sum);
            if sum == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

fn main() {
    let solution = solve(vec![2,7,11,15], 9);
    println!("{:?}", solution);
}

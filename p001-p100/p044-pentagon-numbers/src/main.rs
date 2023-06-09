// https://projecteuler.net/problem=44

use std::collections::HashSet;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let mut max = 1;
    loop {
        let pentagon_numbers: HashSet<u64> = (1..=max).into_iter().map(|n| n * (3 * n - 1) / 2).collect();
        for j in 1..max {
            let pj = j * (3 * j - 1) / 2;
            for k in (j+1)..max {
                let pk = k * (3 * k - 1) / 2;
                let sum = pj + pk;
                let diff = pk - pj;
                if pentagon_numbers.contains(&sum) && pentagon_numbers.contains(&diff) {
                    println!("{}", diff);
                    println!("{:?}", now.elapsed());
                    return;
                }
            }
        }
        max *= 2;
    }
}

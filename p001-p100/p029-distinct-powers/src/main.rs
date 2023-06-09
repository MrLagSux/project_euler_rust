// https://projecteuler.net/problem=29

use std::collections::BTreeSet;
use num_bigint::BigUint;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let mut powers: BTreeSet<BigUint> = BTreeSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            let a = BigUint::from(a as u32);
            let result = a.pow(b);
            powers.insert(result);
        }
    }
    println!("{}", powers.len());
    println!("{:?}", now.elapsed());
}

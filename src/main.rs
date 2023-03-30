use std::collections::HashMap;

fn fibonacci(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if n < 2 {
        return n;
    }

    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, result);
    result
}

fn main() {
    let mut memo = HashMap::new();
    let n = 1000;
    println!("{}", fibonacci(n, &mut memo));
}

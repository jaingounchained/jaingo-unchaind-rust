pub fn log2n(n: &u64) -> Option<u64> {
    if *n == 0 {
        return None;
    } else if *n > 1 {
        return log2n(&(n / 2)).map(|n| n + 1);
    } else {
        return Some(0);
    }
}

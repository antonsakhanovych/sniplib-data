fn binary_search(lo: i64, hi: i64, pred: impl Fn(i64) -> bool) -> i64 {
    let mut lo = lo;
    let mut hi = hi;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if pred(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

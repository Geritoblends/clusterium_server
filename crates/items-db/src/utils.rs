pub fn floor_div(a: i128, b: i128) -> i128 {
    let (q, r) = (a / b, a % b);
    if (r != 0) && ((r > 0) != (b > 0)) {
        q - 1
    } else {
        q
    }
}

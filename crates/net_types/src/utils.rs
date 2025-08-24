fn floori32(a: i32, b: i32) -> i32 {
    let result = a / b;
    if (a % b != 0) && ((a < 0) ^ (b < 0)) {
        result - 1
    } else {
        result
    }
}

fn floori16(a: i16, b: i16) -> i16 {
    let result = a / b;
    if (a % b != 0) && ((a < 0) ^ (b < 0)) {
        result - 1
    } else {
        result
    }
}

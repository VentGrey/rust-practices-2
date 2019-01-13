//---Basic Mathematical Operations

pub fn basic_op(op: char, value1: i32, value2: i32) -> i32 {
    match op {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!("Invalid operator"),
    }
}

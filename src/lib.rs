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

pub fn fizzbuzz(iter: i32) {
    for x in 0..=iter {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", x),
        }
    }
}

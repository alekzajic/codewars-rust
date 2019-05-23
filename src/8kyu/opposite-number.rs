/*
Very simple, given a number, find its opposite.

Examples:

1: -1
14: -14
-34: 34
*/

fn opposite(number: i32) -> i32 {
  - number
}

fn main() {
    println!("{}", opposite(-1));
    println!("{}", opposite(32));
    println!("{}", opposite(-9));
}

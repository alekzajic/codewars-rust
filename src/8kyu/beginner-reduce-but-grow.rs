/*
Given a non-empty array of integers, return the result of multiplying the values together in order. Example:

[1, 2, 3, 4] => 1 * 2 * 3 * 4 = 24
*/

fn grow(array: Vec<i32>) -> i32 {
  let mut res = 1;
  for x in array.iter() {
    res *= x;
  }
  res
}

// best practice
fn grow2(array: Vec<i32>) -> i32 {
    array.iter().product()
}

fn main() {
    println!("{}", grow(vec![1, 2, 3, 4]));
}

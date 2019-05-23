/*
Given a sequence of items and a specific item in that sequence, return the item immediately following the item specified. If the item occurs more than once in a sequence, return the item after the first occurrence. This should work for a sequence of any type.

When the item isn't present or nothing follows it, the function should return nil in Clojure and Elixir, Nothing in Haskell, undefined in JavaScript, None in Python.

next_item([1, 2, 3, 4, 5, 6, 7], 3) //=> 4
next_item(["Joe" "Bob" "Sally"], "Bob") //=> "Sally"

*/
fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    let mut iter = slice.into_iter();
    while let Some(next) = iter.next() {
        if next == &find { return iter.next().cloned(); }
    }
    None
}


fn main() {
    println!("{:?}", next_item(&["Joe", "Bob", "Sally"], "Bob"));
    println!("{:?}", next_item(&[0, 1], 0));
    println!("{:?}", next_item((1..10).collect::<Vec<_>>().as_slice(), 7));
}

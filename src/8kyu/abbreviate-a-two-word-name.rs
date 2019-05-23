/*
Write a function to convert a name into initials. This kata strictly takes two words with one space in between them.

The output should be two capital letters with a dot separating them.

It should look like this:

Sam Harris => `S.H`

Patrick Feeney => `P.F`

*/

fn abbrev_name(name: &str) -> String {
  name.split(' ')
    .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
    .collect::<Vec<_>>()
    .join(".")

}

fn main() {
    println!("{}", abbrev_name("sam harris"));
}

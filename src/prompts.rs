use std::io;
// pass two string and print depending on the answer
pub fn _prompt(positive: &str, negative: &str) {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    if input.trim().eq_ignore_ascii_case("yes") {
        println!("{}", positive);
    } else if input.trim().eq_ignore_ascii_case("no") {
        println!("{}", negative);
    } else {
        println!("invalid command!");
    }
}

// Chapter 8 - Item 2: Strings

fn main() {
    iterating_over_strings();
}

fn concat_strings_using_plus_operator() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{s}");
}

fn iterating_over_strings() {
    // printing a char
    for c in "नमस्ते".chars() {
        println!("{c}")
    }
    println!("--------------");
    // printing bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
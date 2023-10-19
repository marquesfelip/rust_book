fn main() {

    let mut s = String::from("Hello");

    change(s);

    println!("{s}");
}

pub fn change(mut some_string: String) {
    some_string.push_str(" , World!");
}

pub fn is_prime(x: u32) -> (u32, bool) {

    if x <= 1 {
        return (0, false);
    }

    for i in 2..=(x / 2) {
        if x % i == 0 {
            return (0, false);
        }
    }

    (x, true)
}
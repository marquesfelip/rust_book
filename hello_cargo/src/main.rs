fn main() {

    let n: u32 = 10_000;
    let mut count: u32 = 0;
    let mut index: u32 = 0;
    let mut n_prim: u32;
    let mut is_prim: bool;

    loop {
        (n_prim, is_prim) = is_prime(count);
        if is_prim {
            if index == n {
                break;
            } else {
                index += 1;
            }
        }
        count += 1;
    }

    println!("{}", n_prim);
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
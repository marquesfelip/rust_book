fn main() {

    let n = 10_000;
    let mut count = 0;
    let mut index = 0;
    let mut n_prim = 0;
    let mut is_prim = false;

    loop {
        (n_prim, is_prim) = is_prime(count);
        if is_prim {
            println!("{count} é primo");
            if index == n {
                break;
            } else {
                index += 1;
            }
        } else {
            println!("{count} não é primo");
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
            // println!("{x} é divisivel por {i}");
            return (0, false);
        }
    }

    (x, true)
}
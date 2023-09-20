use std::io;

fn main() {
    let mut contador: u128 = 1;
    let mut x: u128 = 1;
    let mut y: u128 = 1;
    let mut total: u128 = 1;
    
    println!("Digite o número de somas de Fibonnaci.");
    let mut qtd_somas: u128 = read_integer();

    while qtd_somas > 0 {
        total = x + y;

        println!("{contador} -> {x} + {y} = {total}");

        y = x;
        x = total;

        contador += 1;
        qtd_somas -= 1;
    }

    // println!("{total}")
}

fn read_integer() -> u128 {
    
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada.");

    input.trim().parse().expect("Falha ao converter entrada para i32.")
}



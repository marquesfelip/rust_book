use std::io;

fn main() {
    
    println!("Escolha a conversão");
    println!("1 - Celsius para Fahrenheit");
    println!("2 - Fahrenheit para Celsius");

    let opcao: i32 = read_integer();
    
    if opcao == 1 {
        println!("Celsius -> Fahrenheit: ");

        let graus: f32 = read_float();
        
        let resultado: f32 = celsius_to_fahrenheit(graus);
        println!("{graus} ºC é igual a {resultado} ºF");
    } else {
        println!("Fahrenheit -> Celsius: ");
        
        let graus: f32 = read_float();

        let resultado = fahrenheit_to_celsius(graus);
        println!("{graus} ºF é igual a {resultado} ºC")
    }
}

fn celsius_to_fahrenheit(num1: f32) -> f32 {
    (num1 * 1.8) + 32.00
}

fn fahrenheit_to_celsius(num1: f32) -> f32 {
    (num1 - 32.00) / 1.8
}

fn read_integer() -> i32 {

    let mut input:String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao receber a entrada.");

    input.trim().parse().expect("Falha ao analisar o número.")
}

fn read_float() -> f32 {
    let mut input:String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao receber a entrada.");

    input.trim().parse().expect("Falha ao analisar o número.")
}
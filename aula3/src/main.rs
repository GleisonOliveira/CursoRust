use std::io;

fn convert_to_int(value: &String) -> i32 {
    value.trim().parse::<i32>().unwrap()
}

fn main() {
    println!("Digite o primeiro número");

    let mut n1 = String::new();
    io::stdin()
        .read_line(&mut n1)
        .expect("Erro ao ler o número");

    println!("Digite o segundo número");

    let mut n2 = String::new();
    io::stdin()
        .read_line(&mut n2)
        .expect("Erro ao ler o número");

    if convert_to_int(&n1) > convert_to_int(&n2) {
        println!("{} é maior que {}", n1, n2);
    } else {
        println!("{} é igual ou menor que {}", n1, n2);
    }
}

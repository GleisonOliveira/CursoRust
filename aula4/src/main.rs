use std::io;

fn main() {
    println!("Digite o número:");

    let mut user_value = String::new();

    io::stdin()
        .read_line(&mut user_value)
        .expect("Erro ao ler valor");

    let sum: i32 = user_value
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .reduce(|acc, v| acc + v)
        .unwrap();

    println!("O valor somado é {}", sum);
}

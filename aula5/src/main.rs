use std::io;

fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Não foi possivel ler o valor");

    user_input
}

fn convert_to_int(input: &String) -> i32 {
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut input: i32 = convert_to_int(&get_user_input());
    let mut factorial: i32 = 1;

    while input > 1 {
        factorial *= input;
        input -= 1;
    }

    println!("O resultado é {}", factorial);
}

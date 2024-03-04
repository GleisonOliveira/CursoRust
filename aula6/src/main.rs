use std::io;

fn convert_to_int(value: &String) -> i32 {
    value.trim().parse::<i32>().unwrap()
}

fn read_line_to_int(message: impl Into<String>) -> i32 {
    println!("{}", message.into());

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Erro ao ler valor");

    convert_to_int(&value)
}

fn main() {
    let averages = read_line_to_int("Digite a quantidade de médias");
    let mut to_recuperate = 0;
    let mut i = 1;

    while averages >= i {
        let student_average = read_line_to_int(format!("Digite a nota do aluno {}:", i));

        if student_average >= 3 && student_average < 6 {
            to_recuperate += 1;
        }

        i += 1;
    }

    println!("O total de alunos em recuperação é {}", to_recuperate);
}

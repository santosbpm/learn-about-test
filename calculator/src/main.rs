use std::io;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn draw_calculator() {
    println!("_______________________________");
    println!("|        Calculadora          |");
    println!("_______________________________");
    println!("|  [7]  [8]  [9]  [÷]  [←]    |");
    println!("|  [4]  [5]  [6]  [x]  [C]    |");
    println!("|  [1]  [2]  [3]  [-]  [AC]   |");
    println!("|  [0]  [.]  [=]  [+]         |");
    println!("_______________________________");
}

fn calculator(operator: Operator, num1: f32, num2: f32) -> f32 {
    match operator {
        Operator::Add => num1 + num2,
        Operator::Subtract => num1 - num2,
        Operator::Multiply => num1 * num2,
        Operator::Divide => num1 / num2,
    }
}

fn get_number() -> f32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor, insira um número válido!");
                continue;
            }
        }
    }
}

fn main() {
    draw_calculator();

    loop {
        println!("\n\nEscolha uma operação:");
        println!("1. Adição (+)");
        println!("2. Subtração (-)");
        println!("3. Multiplicação (*)");
        println!("4. Divisão (/)");
        println!("5. Sair");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Não foi possível ler a entrada!");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido!");
                continue;
            }
        };

        if choice == 5 {
            println!("Saindo...");
            break;
        } else if choice <= 0 && choice >= 5 {
            println!("Escolha inválida!");
            continue;
        }

        println!("\nDigite o primeiro número:");
        let num1 = get_number();
        println!("\nDigite o segundo número:");
        let num2 = get_number();

        if choice == 4 && num2 == 0.0 {
            println!("Erro: Divisão por zero!");
            continue;
        }

        let result = match choice {
            1 => calculator(Operator::Add, num1, num2),
            2 => calculator(Operator::Subtract, num1, num2),
            3 => calculator(Operator::Multiply, num1, num2),
            4 => calculator(Operator::Divide, num1, num2),
            _ => continue,
        };

        println!("Resultado: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(calculator(Operator::Add, 2.0, 2.0), 4.0);
    }
    #[test]
    fn test_subtract() {
        assert_eq!(calculator(Operator::Subtract, 5.0, 3.0), 2.0);
    }
    #[test]
    fn test_multiply() {
        assert_eq!(calculator(Operator::Multiply, 4.0, 3.0), 12.0);
    }
    #[test]
    fn test_divide() {
        assert_eq!(calculator(Operator::Divide, 10.0, 2.0), 5.0);
    }
    #[test]
    fn test_divide_zero() {
        assert_eq!(calculator(Operator::Divide, 10.0, 0.0), std::f32::INFINITY);
    }
}

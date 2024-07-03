use std::io::{self, Write};
use std::str::FromStr;

// Función para leer genéricos
pub fn read_in<T: FromStr>() -> Result<T, T::Err>{
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error al leer la línea");
    input.trim().parse()
}

// Función para reiniciar el programa
pub fn reset_program() -> bool {
    loop {
        println!("¿Deseas hacer otra conversión? (s/n)");
        io::stdout().flush().unwrap();

        let mut r = String::new();
        io::stdin().read_line(&mut r).expect("Error al leer la línea");

        match r.trim().to_lowercase().as_str() {
            "s" | "si" => return true,
            "n" | "no" => return false,
            _ => println!("Por favor responde 'si' o 'no'"),
        }
    }
}

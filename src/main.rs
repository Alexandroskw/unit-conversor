mod conv;
mod mults;
mod si;
mod submult;
mod utils;

use mults::conversor;
use si::internacional;
use utils::{read_in, reset_program};

#[allow(warnings)]
fn main() {
    loop {
        println!("Bienvenido al conversor de unidades");
        println!("¿Qué deseas hacer?:");
        println!("1. Para hacer una conversión de  múltiplos en el S.I.\n2. Para hacer una conversión en cualquiera de ambos sistemas (Internacional, Imperial):");

        match read_in().unwrap_or_else(|_| {
            println!("Introduce una opción");
            0
        }) {
            1 => {
                internacional();
            }
            2 => {
                conversor();
            }
            _ => {
                println!("No es una opción válida");
            }
        }
        if !reset_program() {
            println!("Gracias por usar el conversor");
            break;
        }
    }
}

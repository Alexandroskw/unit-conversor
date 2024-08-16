use crate::conv::{length, temp, weight};
use crate::utils::read_in;
use std::io;

pub fn conversor() {
    println!("Conversor de unidades");
    println!("Este es un conversor de unidades.\nSelecciona el sistema en el que quieras hacer la conversión:");
    println!("1: Sistema Internacional a Imperial\n2: Sistema Imperial a Internacional");

    let sel = match read_in() {
        Ok(valor) => valor,
        Err(_err) => {
            println!("Error al leer la línea");
            return;
        }
    };
    if sel == 1 {
        println!("Seleccionaste el conversor del Sistema Internacional al Imperial");
        println!("¿Qué conversión quieres hacer?:");
        println!("1. Convertir de metros a pies");
        println!("2. Convertir de kilos a libras");
        println!("3. Convertir de Celsius a Farenheit");
        println!("4. Convertir de Celsius a Kelvin");
        println!("5. Convertir de Kelvin a Celsius");
        conversions(sel);
    } else if sel == 2 {
        println!("Seleccionaste el conversor del Sistema Imperial al Internacional");
        println!("¿Qué conversión quieres hacer?:");
        println!("6. Convertir de pies a metros");
        println!("7. Convertir de libras a kilos");
        println!("8. Convertir de Farenheit a Celsius");
        conversions(sel);
    } else {
        println!("Opción no válida.");
    }
}

// Función para convertir del S.I. al Imperial y viceversa
#[allow(warnings)]
fn conversions(selection: u32) {
    match read_in().unwrap_or_else(|_| {
        println!("Error al leer la línea");
        0
    }) {
        1 => {
            println!("Conversor de metros [m] a pies [ft]");
            println!("Introduce el valor a convertir:");

            let mut m = String::new();
            io::stdin().read_line(&mut m).expect("Introduce un número");
            let m = m.trim().parse::<f32>().expect("Error al leer la entrada");
            println!("{} metros equivale a {} pies", m, length::meter_to_feet(m));
        }
        2 => {
            println!("Conversor de kilos [kg] a libras [lb]");
            println!("Introduce el valor a convertir:");

            let mut kg = String::new();
            io::stdin().read_line(&mut kg).expect("Introduce un número");
            let kg = kg.trim().parse::<f32>().expect("Error al leer la entrada");
            println!("{} kilos equivale a {} libras", kg, weight::kg_to_pound(kg));
        }
        3 => {
            println!("Conversor de Celsius [°C] a Farenheit [°F]");
            println!("Introduce el valor a convertir:");

            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Introduce un número");
            let t = temp
                .trim()
                .parse::<f32>()
                .expect("Error al leer la entrada");
            println!("{} °C equivale a {} °F", t, temp::c_to_f(t));
        }
        4 => {
            println!("Conversor de Celsius [°C] a Kelvin [K]");
            println!("Introduce un valor a converir:");

            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Introduce un número");
            let t = temp
                .trim()
                .parse::<f32>()
                .expect("Error al leer la entrada");
            println!("{} °C equivale a {} K", t, temp::c_to_k(t));
        }
        5 => {
            println!("Conversor de Kelvin [K] a Celsius [°C]");
            println!("Introduce un valor a converir:");

            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Introduce un número");
            let t = temp
                .trim()
                .parse::<f32>()
                .expect("Error al leer la entrada");
            println!("{} °C equivale a {} K", t, temp::k_to_c(t));
        }
        6 => {
            println!("Conversor de pies [ft] a metros [m]");
            println!("Introduce un valor a converir:");

            let mut ft = String::new();
            io::stdin().read_line(&mut ft).expect("Introduce un número");
            let f = ft.trim().parse::<f32>().expect("Error al leer la entrada");
            println!("{} pies equivale a {} metros", f, length::feet_to_meter(f));
        }
        7 => {
            println!("Conversor de libras [lb] a kilos [m]");
            println!("Introduce un valor a converir:");

            let mut lb = String::new();
            io::stdin().read_line(&mut lb).expect("Introduce un número");
            let l = lb.trim().parse::<f32>().expect("Error al leer la entrada");
            println!("{} libras equivale a {} kilos", l, length::feet_to_meter(l));
        }
        _ => {
            println!("Opción no válida");
        }
    }
}

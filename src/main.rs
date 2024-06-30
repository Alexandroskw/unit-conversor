use std::io::{self, Write};
use conversor::*;

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

// Función para calcular los submúltiplos de una unidad del S.I.
fn internacional() {
    println!("Conversor de unidades del Sistema Internacional");
    println!("Este es un conversor de sufijos del Sistema Internacional de Unidades");
    println!("Selecciona una unidad para trabajar");
    println!("\n1. Tiempo [s]\n2. Longitud [m]\n3. Masa [kg]\n4. Corriente eléctrica [A]\n");

    loop {
        match read_in().unwrap_or_else(|_| {
            println!("Introduce un número");
            0
            }) {
                1 => {
                    // Sección para submúltiplos de tiempo
                    println!("Seleccionaste tiempo");
                    println!("Introduce un número para iniciar la conversión:");

                    let mut time = String::new();

                    io::stdin()
                        .read_line(&mut time)
                        .expect("Error al leer la línea");

                    let time: f32 = time.trim().parse().expect("Debe ser un número");

                    println!("Introdujiste: {time}");
                    println!("{} segundos son {} minutos", time, tiempo::seg_to_min(time));
                    println!("{} segundos son {} horas", time, tiempo::seg_to_hr(time));
                    println!("{} segundos son {} milisegundos", time, tiempo::seg_to_ms(time));
                    println!("{} segundos son {} microsegundos", time, tiempo::seg_to_us(time));

                    break;
                }
            2 => {
                // Sección para submúltiplos de longitud
                println!("Seleccionaste longitud");
                println!("Indroduce un número para iniciar la conversión");

                let mut long = String::new();

                io::stdin()
                    .read_line(&mut long)
                    .expect("Error al leer la línea");

                let long: f32 = long.trim().parse().expect("Debe ser un número");

                println!("Introdujiste: {long} metros");
                println!("{} metros equivale a {} kilómetros", long, longitud::m_to_km(long));
                println!("{} metros equivale a {} centímetros", long, longitud::m_to_cm(long));
                println!("{} metros equivale a {} milímetros", long, longitud::m_to_mm(long));
                println!("{} metros equivale a {} decímetros", long, longitud::m_to_dm(long));
                println!("{} metros equivale a {} micrometros", long, longitud::m_to_um(long));
                println!("{} metros equivale a {} nanometros", long, longitud::m_to_nm(long));
                
                break;
            }
            3 => {
                // Sección para submúltiplos de masa
                println!("Seleccionaste masa");
                println!("Introduce un número para iniciar la conversión:");

                let mut mass = String::new();

                io::stdin()
                    .read_line(&mut mass)
                    .expect("Error al leer la línea");

                let mass: f32 = mass.trim().parse().expect("Debe ser un número");

                println!("Introdujiste: {mass} gramos");
                println!("{} gramos equivale a {} kilogramos",mass, mass::g_to_kg(mass));
                println!("{} gramos equivale a {} centigramos",mass, mass::g_to_cg(mass));
                println!("{} gramos equivale a {} miligramos",mass, mass::g_to_mg(mass));
                println!("{} gramos equivale a {} decigramos",mass, mass::g_to_dg(mass));
                println!("{} gramos equivale a {} microgramos",mass, mass::g_to_ug(mass));
                println!("{} gramos equivale a {} nanogramos",mass, mass::g_to_ng(mass));

                break;
            }
            4 => {
                println!("Seleccionaste corriente eléctrica"); 
                println!("Introduce un número para la conversión:");

                let mut e = String::new();

                io::stdin().read_line(&mut e).expect("Error al leer la línea");

                let _e: f32 = e.trim().parse().expect("Debe ser un número");

                println!("Introdujiste: {e} amperes");
                println!("{} amperes equivale a {} miliamperes", _e, ampere::a_to_ma(_e));
                println!("{} amperes equivale a {} microamperes", _e, ampere::a_to_ua(_e));
                println!("{} amperes equivale a {} nanoamperes", _e, ampere::a_to_na(_e));

                break;
            } 
            _ => {
                println!("No es una opción válida"); 
            }
        }
    }
}

// Menú del conversor de unidades
fn conversor() {
    println!("Conversor de unidades");
    println!("Este es un conversor de unidades.\nSelecciona el sistema en el que quieras hacer la conversión:");
    println!("1: Sistema Internacional a Imperial\n2: Sistema Imperial a Internacional"); 

    let sel = match read_in() {
        Ok(valor) => valor,
        Err(_err) => {
            println!("Error al leer la línea");
            return
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
    }
    else if sel == 2 {
        println!("Seleccionaste el conversor del Sistema Imperial al Internacional");
        println!("¿Qué conversión quieres hacer?:");
        println!("6. Convertir de pies a metros");
        println!("7. Convertir de libras a kilos");
        println!("8. Convertir de Farenheit a Celsius");
        conversions(sel);
    }
    else {
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
            io::stdin().read_line(&mut temp).expect("Introduce un número");
            let t = temp.trim().parse::<f32>().expect("Error al leer la entrada");
            println!("{} °C equivale a {} °F", t, temp::c_to_f(t));
        }
        4 => {
            println!("Conversor de Celsius [°C] a Kelvin [K]");
            println!("Introduce un valor a converir:");

            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Introduce un número");
            let t = temp.trim().parse::<f32>().expect("Error al leer la entrada");
            println!("{} °C equivale a {} K", t, temp::c_to_k(t));
        }
        5 => {
            println!("Conversor de Kelvin [K] a Celsius [°C]");
            println!("Introduce un valor a converir:");

            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Introduce un número");
            let t = temp.trim().parse::<f32>().expect("Error al leer la entrada");
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

// Función para leer la entrada del usuario
// 'u32' para abarcar la mayor cantidad de números posibles, flotantes o enteros
fn read_in() -> Result<u32, std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

// Función para volver a repetir el programa
fn reset_program() -> bool {
    loop {
        println!("¿Deseas hacer otra conversión? (s/n):");
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

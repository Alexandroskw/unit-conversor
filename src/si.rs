use crate::submult::*;
use crate::utils::read_in;
use std::io;

#[allow(warnings)]
pub fn internacional() {
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
                // Uso de tuplas para la simplicidad del programa. Al crear una tupla se debe
                // de indicar el tipo de cada uno de los elementos (string y una función de que
                // regresa un 'f32' en este caso)
                let sub: [(&str, fn(f32) -> f32); 4] = [
                    ("minutos", tiempo::seg_to_min),
                    ("horas", tiempo::seg_to_hr),
                    ("milisegundos", tiempo::seg_to_ms),
                    ("microsegundos", tiempo::seg_to_us),
                ];

                io::stdin()
                    .read_line(&mut time)
                    .expect("Error al leer la línea");

                let time: f32 = time.trim().parse().expect("Debe ser un número");

                println!("Introdujiste: {time}");
                for &(unidad, func) in sub.iter() {
                    let resultado = func(time);
                    println!("{} segundos son {} {}", time, resultado, unidad);
                }

                break;
            }
            2 => {
                // Sección para submúltiplos de longitud
                println!("Seleccionaste longitud");
                println!("Indroduce un número para iniciar la conversión");

                let mut long = String::new();
                let sub: [(&str, fn(f32) -> f32); 6] = [
                    ("kilómetros", longitud::m_to_km),
                    ("centímetros", longitud::m_to_cm),
                    ("milímetros", longitud::m_to_mm),
                    ("decímetros", longitud::m_to_dm),
                    ("micrómetros", longitud::m_to_um),
                    ("nanómetros", longitud::m_to_nm),
                ];

                io::stdin()
                    .read_line(&mut long)
                    .expect("Error al leer la línea");

                let long: f32 = long.trim().parse().expect("Debe ser un número");

                println!("Introdujiste: {long} metros");
                for &(unidad, func) in sub.iter() {
                    let resultado = func(long);
                    println!("{} metros equivalen a {} {}", long, resultado, unidad);
                }

                break;
            }
            3 => {
                // Sección para submúltiplos de masa
                println!("Seleccionaste masa");
                println!("Introduce un número para iniciar la conversión:");

                let mut mass = String::new();
                let sub: [(&str, fn(f32) -> f32); 6] = [
                    ("kilogramos", mass::g_to_kg),
                    ("centigramos", mass::g_to_cg),
                    ("miligramos", mass::g_to_mg),
                    ("decigramos", mass::g_to_dg),
                    ("microgramos", mass::g_to_ug),
                    ("nanogramos", mass::g_to_ng),
                ];

                io::stdin()
                    .read_line(&mut mass)
                    .expect("Error al leer la línea");

                let mass: f32 = mass.trim().parse().expect("Debe ser un número");
                println!("Introdujiste: {mass} gramos");
                for &(unidad, func) in sub.iter() {
                    let resultado = func(mass);
                    println!("{} gramos equivalen a {} {}", mass, resultado, unidad);
                }

                break;
            }
            4 => {
                println!("Seleccionaste corriente eléctrica");
                println!("Introduce un número para la conversión:");

                let mut e = String::new();
                let sub: [(&str, fn(f32) -> f32); 3] = [
                    ("miliamperes", ampere::a_to_ma),
                    ("microamperes", ampere::a_to_ua),
                    ("nanoamperes", ampere::a_to_na),
                ];

                io::stdin()
                    .read_line(&mut e)
                    .expect("Error al leer la línea");

                let _e: f32 = e.trim().parse().expect("Debe ser un número");
                println!("Introdujiste: {e} amperes");

                for &(unidad, func) in sub.iter() {
                    let resultado = func(_e);
                    println!("{} amperes equivalen a {} {}", _e, resultado, unidad);
                }
                break;
            }
            _ => {
                println!("No es una opción válida");
            }
        }
    }
}

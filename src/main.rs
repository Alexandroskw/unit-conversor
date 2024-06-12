use std::io;
use conversor::*;

fn main() {
    println!("Bienvenido al conversor de unidades");
    println!("¿Qué deseas hacer?:");
    println!("'1' para hacer una conversión de  múltiplos en el S.I.\n'2' para hacer una conversión en cualquiera de ambos sistemas (Internacional, Imperial):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la línea");
        let input = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Introduce un número");
                continue;
            }
        };

        match input {
            1 => {
                internacional();
                break;
            }
            2 => {
                conversor();
                break;
            }
            _ => {
                println!("No es una opción válida");
            } 
        }
    }
}

fn internacional() {
    println!("Conversor de unidades del Sistema Internacional");
    println!("Este es un conversor de sufijos del Sistema Internacional de Unidades");
    println!("Selecciona una unidad para trabajar");
    println!("\n1. Tiempo [s]\n2. Longitud [m]\n3. Masa [kg]\n4. Corriente eléctrica [A]\n");

    loop {
        let mut unit = String::new();
        io::stdin().read_line(&mut unit).expect("Error al leer la línea");

        let input = match unit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Introduce un número");
                continue;
            }
        };

        match input {
            1 => {
                println!("Seleccionaste tiempo");
                println!("Introduce un número para iniciar la conversión:");

                let mut time = String::new();

                io::stdin()
                    .read_line(&mut time)
                    .expect("Error al leer la línea");

                let time: f32 = time.trim().parse().expect("Debe ser un número");
                let mut c: f32;
                let mut c1: f32;
                let mut c2: f32;
                let mut c3: f32;
                println!("Introdujuste {time} segundos."); 
                println!("{time} segundos equivale a {c} minutos", c = (time*1.0000)/60.0000);
                println!("{time} segundos equivale a {c1} horas", c1 = (time*1.0000)/3600.0000);
                println!("{time} segundos equivale a {c2} milisegundos", c2 = (time*1000.0000)/1.0000);
                println!("{time} segundos equivale a {c3} microsegundos", c3 = (time*1000000.0000)/1.0000);
            }
            2 => {
                println!("Seleccionaste longitud");
                println!("Indroduce un número para iniciar la conversión");

                let mut long = String::new();

                io::stdin()
                    .read_line(&mut long)
                    .expect("Error al leer la línea");

                let long: f32 = long.trim().parse().expect("Debe ser un número");

                let c: f32;
                let c1: f32;
                let c2: f32;
                let c3: f32;
                let c4: f32;
                let c5: f32;

                println!("Introdujiste: {long} metros");
                println!("{long} metros equivale a {c} kilometros", c = long/1000.0000);
                println!("{long} metros equivale a {c1} centímetros", c1 = (long*1000.0000)/10.0000);
                println!("{long} metros equivale a {c2} milímetros", c2 = (long*1000.0000)/1.0000);
                println!("{long} metros equivale a {c3} decímetros", c3 = (long*1000.0000)/100.0000);
                println!("{long} metros equivale a {c4} micrómetros", c4 = long*1_000_000.0);
                println!("{long} metros equivale a {c5} nanómetros", c5 = long*1000_000_000.0);
            }
            3 => {
                println!("Seleccionaste masa");
                println!("Introduce un número para iniciar la conversión:");

                let mut mass = String::new();

                io::stdin()
                    .read_line(&mut mass)
                    .expect("Error al leer la línea");

                let mass: f32 = mass.trim().parse().expect("Debe ser un número");

                let c: f32;
                let c1: f32;
                let c2: f32;
                let c3: f32;
                let c4: f64;
                let c5: f64;

                println!("Introdujiste: {mass} gramos");
                println!("{mass} gramos equivale a {c} kilogramos", c = mass/1000.0000);
                println!("{mass} gramos equivale a {c1} centigramos", c1 =(mass*1000.0000)/10.0000);
                println!("{mass} gramos equivale a {c2} miligramos", c2 =(mass*1000.0000)/1.0000);
                println!("{mass} gramos equivale a {c3} decigramos", c3 =(mass*1000.0000)/100.0000);
                println!("{mass} gramos equivale a {c4} microgramos", c4 = mass*1_000_000.0);
                println!("{mass} gramos equivale a {c5} nanogramos", c5 = mass*1000_000_000.0);

                break;
            }
            4 => {
                println!("Seleccionaste corriente eléctrica"); 
                println!("Introduce un número para la conversión:");

                let mut e = String::new();

                io::stdin().read_line(&mut e).expect("Error al leer la línea");

                let e: f32 = e.trim().parse().expect("Debe ser un número");
                let c: f32;
                let c1: f32;
                let c2: f32;

                println!("Introdujiste: {e} amperes");
                println!("{e} amperes equivale a {c} miliamperes", c = e*1_000.0000);
                println!("{e} amperes equivale a {c1} microamperes", c1 = e*1_000_000.0);
                println!("{e} amperes equivale a {c2} nanoamperes", c2 = e*1000_000_000.0);

                break;
            } 
            _ => {
                println!("No es una opción válida"); 
            }
        }
    }
}

fn conversor() {
    println!("Conversor de unidades");
    println!("Este es un conversor de unidades.\nSelecciona el sistema en el que quieras hacer la conversión:");
    println!("1: Sistema Internacional a Imperial\n2: Sistema Imperial a Internacional");

    let mut o = String::new();
    io::stdin().read_line(&mut o).expect("Introduce un número");
    let o = o.trim().parse().expect("Error al leer la línea"); 

    match o {
        1 => {
            println!("Seleccionaste la conversión del Sistema Internacional al Imperial");
            println!("Selecciona una opción:");
            println!("1. Convertir de metros a pies\n2. Convertir de pies a metros");
            println!("3. Convertir de kilos a libras\n4. Convertir de libras a kilos");
            println!("5. Convertir de Celsius a Farenheit\n6. Convertir de Farenheit a Celsius");
            println!("7. Convertir de Celsius a Kelvin\n8. Convertir de Kelvin a Celsius");

            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Introduce un número");
            let s = s.trim().parse::<u32>().expect("Error al leer la línea"); 

            conversions(s);
        }
        2 => {
            println!("Has elegido la segunda opción");
        }
        _ => {
            println!("Error");
        }
    }
}

fn conversions(selection: u32) {
    match selection {
        1 => {
            println!("Seleccionaste convertir de metros a pies");
            println!("Ingresa los metros a convertir:");
            let mut m = String::new();
            io::stdin().read_line(&mut m).expect("Introduce un número");
            let m = m.trim().parse::<f32>().expect("Error al leer la línea");
            println!("Has puesto {m} metros");

            println!("{} metros son {} pies", m, length::meter_to_feet(m));
        }
        2 => {
            println!("Seleccionaste convertir de pies a metros");
            println!("Ingresa los pies a convertir:");
            let mut ft = String::new(); 
            io::stdin().read_line(&mut ft).expect("Introduce un número");
            let ft = ft.trim().parse::<f32>().expect("Error al leer la línea");
            println!("Has puesto {ft} pies");

            println!("{} pies son {} metros", ft, length::feet_to_meter(ft));
        }
        3 => {
            println!("Seleccionaste convertir de kilos a libras");
            println!("Ingresa los kilos a convertir:");
            let mut kg = String::new(); 
            io::stdin().read_line(&mut kg).expect("Introduce un número");
            let kg = kg.trim().parse::<f32>().expect("Error al leer la línea");
            println!("Has puesto {kg} kilogramos");

            println!("{} kilogramos son {} libras", kg, weight::kg_to_pound(kg));
        }
        4 => {
            println!("Seleccionaste convertir de libras a kilos");
            println!("Ingresa las libras a convertir:");
            let mut lb = String::new(); 
            io::stdin().read_line(&mut lb).expect("Introduce un número");
            let lb = lb.trim().parse::<f32>().expect("Error al leer la línea");
            println!("Has puesto {lb} kilogramos");

            println!("{} libras son {} kilogramos", lb, weight::pound_to_kg(lb));
        }
        5 => {
            println!("Seleccionaste convertir de grados celsius a farenheit");
            println!("Ingresa los grados celsius a convertir:");
            let mut c = String::new(); 
            io::stdin().read_line(&mut c).expect("Introduce un número");
            let c = c.trim().parse::<f32>().expect("Error al leer la línea");
            println!("Has puesto {c} grados celsius");

            println!("{} celsius son {} farenheit", c, temp::c_to_f(c));
        }
        6 => {
            println!("Seleccionaste convertir de grados farenheit a celsius");
            println!("Ingresa los grados farenheit a convertir:");
            let mut f = String::new(); 
            io::stdin().read_line(&mut f).expect("Introduce un número");
            let f = f.trim().parse::<f32>().expect("Error al leer la línea");
            println!("Has puesto {f} grados celsius");

            println!("{} farenheit son {} celsius", f, temp::f_to_c(f));
        }
        _ => {
            println!("No es una opción válida");
        }
    }
}

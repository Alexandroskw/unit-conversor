use std::io;

fn main() {
    println!("Bienvenido al conversor de unidades");
    println!("Selecciona con qué sistema utilizas");
    println!("S.I. o Inglés");
    println!("'1' para el sistema internacional, '2' para inglés");

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
                println!(">Introdujiste la segunda opcion");
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
    println!("Este esn un conversor de sufijos del Sistema Internacional de Unidades");
    println!("Selecciona una unidad para trabajar");
    println!("\n1. Tiempo [s]\n2. Longitud [m]\n3. Masa [kg]\n4. Corriente eléctrica [A]\n5. Temperatura [K]\n6. Cantidad de sustancia [mol]\n7. Intensidad luminosa [cd]");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Error al leer la línea");

    let input = unit.trim().parse().expect("Introduce un número");

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
        }
        4 => {
            println!("Seleccionaste corriente eléctrica"); 
            println!("Introduce un número para la conversión:");

            let mut e = String::new();

            io::stdin().read_line(&mut e).expect("Error al leer la línea");

            let e: f32 = e.trim().parse().expect("Debe ser un número");
            let c: f32;

            println!("Introdujiste: {e} amperes");
            println!("{e} amperes equivale a {c} miliamperes");
        }
        5 => {
            println!("Seleccionaste temperatura"); 
        }
        6 => {
            println!("Seleccionaste cantidad de sustancia"); 
        }
        7 => {
            println!("Seleccionaste intensidad luminosa"); 
        }
        _ => {
            println!("No es una opción válida"); 
        }
    }
}

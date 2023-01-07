fn usando_match(x: i32) {
    match x {
        1 => {
            let x = 10;
            println!("{}", x + 10);
        }
        2 | 4 | 6 | 8 => println!("El numero es par"),
        20..=30 => println!("Entre el 20 y 30"),
        _ => println!("No hay coincidencia"),
    }
}

enum TipoDeContacto {
    Personal(String),
    Colega(u64),
    Cliente(String),
}
fn mostrar_contacto(contact: TipoDeContacto) {
    match contact {
        TipoDeContacto::Personal(email) => {
            println!("Personal: {}", email);
        }
        TipoDeContacto::Colega(edad) => {
            println!("colega: {}", edad);
        }
        TipoDeContacto::Cliente(nombre) => {
            println!("cliente: {}", nombre);
        }
    }
}

struct Color {
    rojo: u8,
    verde: u8,
    azul: u8,
}

fn clasifica_color(c: Color) {
    match c {
        Color {
            rojo: 0,
            verde: 0,
            azul: 0,
        } => {
            println!("Negro");
        }
        Color {
            rojo: 255,
            verde: 255,
            azul: 255,
        } => {
            println!("Blanco");
        }
        Color {
            rojo: r,
            verde: 0,
            azul: 0,
        } => {
            println!("Rojo {}", r);
        }
        Color {
            rojo: 0,
            verde: v,
            azul: 0,
        } => {
            println!("Verde {}", v);
        }
        Color {
            rojo: 0,
            verde: 0,
            azul: b,
        } => {
            println!("Azul {}", b);
        }
        Color {
            rojo: r,
            verde: v,
            azul: b,
        } => {
            println!("combinancion color {} ,{}, {}", r, v, b);
        }
    }
}
fn main() {
    let x = 1;
    match x {
        1 => println!("Uno"),
        2 => println!("dos"),
        3 => println!("tres"),
        4 => println!("cuatro"),
        5 => println!("Cinco"),
        _ => println!("No hay coincidencia"),
    }

    usando_match(1);
    usando_match(6);
    usando_match(25);
    usando_match(100);

    let boolean = true;
    let mensaje = match boolean {
        false => "Es falso!",
        true => "Es verdadero",
    };
    println!("{} -> {}", boolean, mensaje);

    mostrar_contacto(TipoDeContacto::Cliente(String::from("wlopezob")));
    mostrar_contacto(TipoDeContacto::Personal(String::from("w@h.com")));

    let color1 = Color {
        rojo: 12,
        verde: 0,
        azul: 0,
    };
    clasifica_color(color1);

   let color2 = Color {
        rojo: 12,
        verde: 10,
        azul: 30,
    };
    clasifica_color(color2);
  
}

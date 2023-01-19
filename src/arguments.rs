use std::env;

fn arguments() {
    let argumentos: Vec<String> = env::args().collect();
    dbg!(&argumentos);

    if (argumentos.len() > 2) {
        let parametro1 = &argumentos[1];
        let parametro2 = &argumentos[1];

        println!("El primer parametro es: {}", parametro1);
        println!("El segundo parametro es: {}", parametro2);
    } else {
        println!("El formato es incorrecto");
    }
}

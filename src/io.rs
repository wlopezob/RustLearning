use std::{io, io::BufRead};
fn io() {
    let mut texto = String::new();
    let stdin = io::stdin();
    println!("Ingrese cualquier texto");
    stdin
        .read_line(&mut texto)
        .expect("Ha ocurrido un error al leer el texto");
    println!("El texto es: {}", texto);

    //otra forma de obtener
    println!("Ingrese texto01:");
    let texto01 = stdin.lock().lines().next().unwrap().unwrap();

   println!("Ingrese texto02:");
    let texto02 = stdin.lock().lines().next().unwrap().unwrap();
  println!("{} {}", texto01, texto02);
}

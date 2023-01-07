
use std::fs;

fn main() {
let contenido = fs::read_to_string("src/bucles.rs").unwrap();

  println!("El texto es: {}", contenido);
  
}

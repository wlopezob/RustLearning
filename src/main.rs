fn usando_match(x: i32) {
  match x {
    1 => {
      let x= 10;
      println!("{}", x+10);
    },
    2 | 4 | 6 | 8 =>  println!("El numero es par"),
    20..=30 => println!("Entre el 20 y 30"),
    _=>println!("No hay coincidencia"),
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
}


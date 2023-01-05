fn flujo_control() {
    let x = 20;
  let y = 9;
  if x < y {
    println!("el numero {} es menor que {}", x, y);
  } else {
    println!("el numero {} es mayor que {}", x, y);
    
  }

   if x != y {
    println!("el numero {} es diferente que {}", x, y);
  } else {
    println!("el numero {} es igual que {}", x, y);
    
  }

   if x % y == 0 {
    println!("el numero {} es divisible que {}", x, y);
  } else if x %2 == 0 {
    println!("el numero {} es par", x);
  } else {
    println!("el numero {} no es divisible que {}", x, y);
  }

  let mensaje = if x < y {"el valor de x es menor que y"} else {"el valor de x es mayor o igual que 10"};

    println!("mensaje: {}", mensaje);
}

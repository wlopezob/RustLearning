fn functions() {
    mi_funcion(1, false);
    println!("valor funcion: {}", get_10());

    let x = 10;
    let y = {
        // se sobreescribe el valor
        let x = 1;
        // retorna la multiplicacion
        x * 4
    };
    println!("valor y {}", y);
    println!("valor x {}", x);
}

fn mi_funcion(x: i8, y: bool) {
    println!("el valor de la variable x es: {}", x);
    println!("el valor de la variable y es: {}", y);
}

fn get_10() -> i32 {
    println!("obtiene valor");
    // si no se pone el ; entons es una sentencia de return
    10
}

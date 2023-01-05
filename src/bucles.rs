fn bucles() {
    //bucle infinito
    loop {
        println!("Subscribete y dale like!");
        //termina el ciclo
        break;
    }
    //asignando a un valor el resultado del bucle
    let x = loop {
        break 2 * 5;
    };
    println!("valor x: {}", x);

    let mut counter = 0;
    let y = loop {
        counter += 1;
        if counter == 10 {
            break 2 * counter;
        }
    };
    println!("counter: {}", y);

    // WHILE
    // no se puede usar para retornar valores como el loop
    let mut contador = 0;
    while contador < 10 {
        contador += 1;
    }
    println!("contador: {}", contador);

    contador = 0;
    while contador < 10 {
        contador += 1;
        break;
    }
    println!("contador: {}", contador);

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("El valor es: {}", arr[index]);
        index += 1;
    }

    println!("FOR");
    //For
    for element in arr.iter() {
        println!("El valor es: {}", element);
    }

    for number in 1..5 {
        println!("El numero es: {}", number);
    }

    for number in (1..5).rev() {
        println!("El numero es: {}", number);
    }
}

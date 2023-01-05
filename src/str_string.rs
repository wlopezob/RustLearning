fn str_string() {
    //stack: guardamos informacion de un tamaño fijo en la ejecucion de un programa
    //heap: guardamos informacion de un tamaño dinamico en la ejecucion de un programa

    let saludo = "hola mundo"; //se guarda en stack
    let mut saludo1 = String::from("hola mundo"); //se guarda en el heap, se asemeja al stringbuffer

    println!("saludo: {}", saludo);
    println!("saludo1: {}", saludo1);

    println!("tamaño: {}", saludo.len());
    println!("tamaño: {}", saludo1.len());

    //el tipo de dato &str no tiene el metodo capacity
    //println!("capacity: {}", saludo.capacity());
    println!("capacity: {}", saludo1.capacity());
    saludo1.push_str(" desde RUST");
    println!("{}", saludo1);
    println!("capacity: {}", saludo1.capacity());

    //paso por referencia
    saludar(&saludo1);
    saludar01(saludo1);
}

fn saludar(saludo: &str) {
    println!("{}", saludo);
}

fn saludar01(saludo: String) {
    println!("{}", saludo);
}

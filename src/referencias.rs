fn referencias() {
    let sub = String::from("Hola mundo");
    //el owner de sub se paso a la funcion
    //let long = calcular_longitud(sub);

    //trabajando con tuplas
    let (long, sub) = calcular_longitud01(sub);
    println!("La longitud de '{}' es: {}", sub, long);

    //pasando por referencia
    let long = calcular_longitud02(&sub);
    println!("La longitud de '{}' es: {}", sub, long);

    let mut sub01 = String::from("Hola mundo");
    //mutable por referencia
    let long = calular_longitud03(&mut sub01);
    println!("La longitud de '{}' es: {}", sub01, long);

    //pasando la referencia y tbm referencia mutable
    let mut h = String::from("hola");
    let h1 = &h;
    let h2 = &h;
    println!("{}, {}", h1, h2);
    let h3 = &mut h;
    println!("{}", h3);

    //referencia colgante, da error, xq el owner esta eliminado dentro de la referencia
    //let rf = referencia_colangte();
}

fn calcular_longitud(s: String) -> usize {
    s.len()
}

fn calcular_longitud01(s: String) -> (usize, String) {
    (s.len(), s)
}

fn calcular_longitud02(s: &String) -> usize {
    s.len()
}

fn calular_longitud03(s: &mut String) -> usize {
    s.push_str(" modificado");
    s.len()
}

fn referencia_colangte() -> &String {
    let h = String::from("hola");
    &h
} //la variable h se elimina fuera de esta funcion, por eso su referencia queda en el vacio. obteniendo error

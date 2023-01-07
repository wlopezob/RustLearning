use std::collections::HashMap;
fn hashmap() {
    let mut salas = HashMap::new();
    salas.insert("Javascripters".to_string(), 50);
    println!("{:?}", salas.get("Javascripters"));
    salas.insert("Rustaceans".to_string(), 80);
    println!("{:?}", salas);
    //inserta la key si no existe
    let v = salas.entry("Rustaceans".to_string()).or_insert(50);
    // obtiene 80 xq ya existe el key en el hashmap
    println!("valor: {:?}", v);
    salas.entry("Javeros".to_string()).or_insert(500);
    println!("{:?}", salas);

    //iterar
    for (key, value) in &salas {
        println!("{} --> {}", key, value);
    }

    //hashmap con prestamos y referencia
    let mut hash = HashMap::new();
    let mi_llave = String::from("millave");
    let mi_valor = String::from("mivalor");
    hash.insert(&mi_llave, &mi_valor);
    println!("mi hash {:?}", hash);
    println!("mi valor: {}", mi_valor);
}

fn vector() {
    //Los vectores son como un array de objetos pero que se pueden añadir items
    let mut vector: Vec<i32> = Vec::new();
    vector.push(10);
    vector.push(20);
    println!("{:?}", vector);
    println!("{:?}", vector.pop());
    println!("{:?}", vector);

    let mut vector01 = vec![1, 2, 3, 4, 5];
    vector01.push(6);
    println!("{:?}", vector01);

    let vector02 = vec![String::from("gola"); 100];
    println!("{:?}", vector02);

    let vector03 = vec![1, 2, 3, 4, 5];
    let elemento = vector03[2];
    println!("elemento: {:?}", elemento);

    let elemento_ref = &vector03[2];
    println!("elementoRef: {:?}", elemento_ref);

    let elemento02 = &vector03[2..4];
    println!("elemento02: {:?}", elemento02);

    let mut miv = vec![1, 2, 3, 4, 5];
    let elemento = miv.get(2);
    println!("mi elemento: {:?}", elemento);
    let elemento = miv.get(100);
    println!("mi elemento: {:?}", elemento);

    // iterar
    // tomando referencia
    for i in &miv {
        println!("una referencia: {}", i);
    }

    for i in &mut miv {
        *i += 10;
        println!("Una referencia mutable a {}", i);
    }

    for i in miv {
        //valor de variable vector movida
        println!("Tomando pertenencia del vector y su elemento: {}", i);
    } //a partir de aqui ya no podemos usar miv, debido a que no pasamos la referencia

    let diferente = vec![
        DiferenteTipoDato::Int(3),
        DiferenteTipoDato::Float(0.5),
        DiferenteTipoDato::Text(String::from("hola"))
    ];

    println!("diferente tipo dato: {:?}", diferente);
}

#[derive(Debug)]
enum DiferenteTipoDato {
    Int(i32),
    Float(f64),
    Text(String),
}

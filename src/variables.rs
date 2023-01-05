fn variables() {
    let mut x: u8 = 10;
    println!("{}", x);
    x = 20;
    println!("{}", x);
    println!("Hello, world!");

    // las constantes deben de tener un tipo de datos
    const MI_CONSTANTE: u32 = 29_2;
    println!("{}", MI_CONSTANTE);

    //sombreo
    let y = 10;
    println!("{}", y);
    let y = y * 2;
    println!("{}", y);

    let m = "hello world";
    println!("{}", m);
    let m = m.len();
    println!("cantida de digitos: {}", m);

    //una variable mut no puede cambiarse el tipo de dato
    /*
    let mut n1 = "hello world";
    println!("{}", n1);
    n1 = n1.len(); // ERROR
    println!("cantida de digitos: {}", n1);
    */
}

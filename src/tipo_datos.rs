fn tipo_datos() {
    let x = 10;
    println!("{}", x);

    let y: i8 = "100".parse().expect("No es numero");
    println!("numero: {}", y);

    let z = 10.32;
    println!("numero: {}", z);

    //no se puede sumar dos numeros de diferentes tipos
    //let a = 10.32 + 4;

    let x = true;
    println!("boolean: {}", x);

    let c = 'a';
    println!("caracter: {}", c);
}

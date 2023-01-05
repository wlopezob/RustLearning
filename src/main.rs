fn main() {
    //Reglas:
    //1. Cada valor contiene un owner(variable)
    //2. Solo un owner por valor
    //3. Cuando un owner sale del ambito o scope es eliminado

    //Casos
    // Regla 2
    let x = 10;
    let y = 20;
    //se puede hacer esto xq se usa dentro del stack
    let z = x;
    println!("{},{},{}", x, y, z);
    println!("{:p},{:p},{:p}", &x, &y, &z);

    let a: String = String::from("hola mundo");
    let b: String = String::from("hola mundo");
    //esto gener un error, xq el valor de a esta siendo movido a otro owner, rompe la regla 2
    //let c : a;
    //podemos clonarlo
    let c = a.clone();

    println!("{},{},{}", a, b, c);
    println!("{:p},{:p},{:p}", &a, &b, &c);

    //variable del heap
    let hola = String::from("hola mundo");
    saludar(hola);
    //esto arroja error xq la variable hola fue eliminada en la funcion saludar
    //println!("{}", hola);

    //variable del heap
    let hola = String::from("hola mundo");
    saludar01(&hola); //estamos pasando la referencia de la variable hola
    println!("saludar01: {}", hola);

    //variables del stack
    let x = 10;
    let y = 20;
    suma(x, y); //se hace una copia de las variables x, y
    println!("{}, {}", x, y);
    println!("{:p},{:p}", &x, &y);

    let saludo = recibir_saludo();
    println!("recibir saludo: {}", saludo);
    println!("{:p}", &saludo);
}

fn saludar(hola: String) {
    println!("hola {}", hola);
} // la variable hola es eliminada

fn suma(x: i32, y: i32) {
    println!("suma: {}", x + y);
    println!("{:p},{:p}", &x, &y);
} //la variable x e y son eliminados dentro de este scope

fn saludar01(hola: &str) {
    println!("hola {}", hola);
}

fn recibir_saludo() -> String {
    let saludar = String::from("hola mundo");
    println!("fn recibir_saludo: {:p}", &saludar);
    saludar
}

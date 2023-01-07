struct Circulo {
    x: f64,
    y: f64,
    radio: f64,
}

struct Cuadrado {
    lado: f64,
}
// interfaces
trait Area {
    fn area(&self) -> f64;
}
impl Area for Circulo {
    fn area(&self) -> f64 {
        3.1416 * self.radio * self.radio
    }
}
impl Area for Cuadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
}

// funcion para trabajar con trait
fn mostrar_area<T: Area>(figura: T) {
    println!("la figura tiene un area: {}", figura.area());
}

//extender tipos de datos
trait Sumas {
    fn sumar10(&self) -> i32;
    fn doblar(&self) -> i32;
}

impl Sumas for i32 {
    fn sumar10(&self) -> i32 {
        self + 10
    }
    fn doblar(&self) -> i32 {
        self * 2
    }
}
fn traits() {
    let c = Circulo {
        x: 0.0,
        y: 0.0,
        radio: 3.5,
    };
    println!("el area del circulo es: {}", c.area());

    let cu = Cuadrado { lado: 10.0 };
    println!("el area del cuadraro es {}", cu.area());

    mostrar_area(c);
    mostrar_area(cu);

    let num = 28;
    println!("{},{}", num.sumar10(), num.doblar());
}

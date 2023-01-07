use std::fmt::Debug;

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

struct Info<T> {
    valor: T,
}

impl<T: Debug> Info<T> {
    fn mostrar_valor(&self) {
        println!("El valor es: {:?}", &self.valor);
    }

    fn mostrar_valor2<U: Debug>(&self, x: U) {
        println!("El valor por parametro es: {:?}", x);
    }
}
fn recibe_cualquier_cosa<T: Debug>(x: T) {
    println!("cualquiercosa: {:?}", x);
}
fn generico() {
    let x: Option<String> = Option::Some("hola mundo".to_string());
    println!("Valor de x: {:?}", x);

    let t1: Info<i32> = Info { valor: 32 };
    println!("El valor es: {}", t1.valor);

    let t1: Info<String> = Info {
        valor: "hola mundo".to_string(),
    };
    println!("El valor es: {}", t1.valor);

    recibe_cualquier_cosa(123);

    let t: Info<i32> = Info{valor:100};
    t.mostrar_valor();
  t.mostrar_valor2("valor");
}

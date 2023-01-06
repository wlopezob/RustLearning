#[derive(Debug)]
enum Genero {
  Masculino,
  Femenino
}

#[derive(Debug)]
struct Persona{
  nombre: String,
  genero: Genero
}


enum Velocidad {
  Lenta = 10,
  Media = 20,
  Rapida = 50
}

enum Dificultad {
  Facil = 1,
  Medio, //rust le asigna el valor 2 automaticamente
  Dificil //rust le asigna el valor 3 automaticamente
}

//enum con alias
#[derive(Debug)]
enum EnumConDiversasOpcionNombreLargo {
  Suma, Resta
}

//creando un alias a la enumeracion
type Operaciones = EnumConDiversasOpcionNombreLargo;

#[derive(Debug)]
enum Value{
  Number(f64),
  Str(String),
  Bool(bool, bool),
  Struct{x:i32, y:i32}
}


//eumeracion option
enum Option<T>{
  Some(T), //puede recibir cualquier valor
  None //o ninguno
}

fn enumeraciones() {
    let varon = Genero::Masculino;
    let mujer = Genero::Femenino;

  println!("{:?}", varon);
  println!("{:?}", mujer);

  let p1 = Persona{
    nombre : String::from("carlos"),
    genero : Genero::Masculino
  };

let p2 = Persona{
    nombre : String::from("ernesto"),
    genero : Genero::Masculino
  };

println!("{:?}", p1);
println!("{:?}", p2);

  let lenta = Velocidad::Lenta;
  let lenta = lenta as u32;
  println!("velocidad {:?}", lenta);

  let dificultad = Dificultad::Dificil;
  let dificultad = dificultad as u32;
  println!("dificultad {:?}", dificultad);

  let suma = Operaciones::Suma;
  println!("operaciones  {:?}", suma);
  
  let n = Value::Number(2.3);
  let string = Value::Str(String::from("hola mundo"));
  let b = Value::Bool(true, false);
  let s = Value::Struct{x:10, y:32};
  println!("n {:?} string {:?} b {:?} s {:?}", n, string, b, s);

  //invocando al enum option some
  let numero_some = Some(4);
  let string_some = Some("hola mundo");
  let string_some2 = Some(String::from("hola mundo"));

  println!("numero_some  {:?}", numero_some);
  println!("string_some  {:?}", string_some);
  println!("string_some2  {:?}", string_some2);
  
}


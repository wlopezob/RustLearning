fn datos_compuestos() {
    let tup = (12, 4.5, "hola");
    println!("valor tupla:{:?}", tup);

    //declarando tupla
    let tup1: (u8, f32, &str) = (12, 3.5, "hola");
    println!("valor tupla01:{:?}", tup1);
  let (x, y, z) = tup1;
  println!("valor x:{}",x);
  println!("valor y:{}",y);
  println!("valor z:{}",z);

  let xx = tup1.0;
  println!("valor xx:{}",xx);

  let mut tup2: (u8, f32, &str) = (12, 3.5, "hola");
  tup2.0  = 1;
  println!("valor indice 0:{}",tup2.0);

  /*Arreglos*/

  // todos los datos tienen que ser del mismo tipo
  let arr = [1,2,3,4,5];
  println!("arreglo {:?}", arr);

  let meses = ["enero", "febrero", "marzo"];
  println!("meses {:?}", meses);

  // declarando tipo de datos y de 05 de tamaño
  // se tiene que declarar los 05 elementos
  let numbers : [i32; 5] = [1,2,3,4,8];
  println!("numeros {:?}", numbers);

  //declarando arrays con valores iniciales
  //05 elementos con valor de "hola"
  let num01 = ["hola";5];
  println!("num01 {:?}", num01);

  let car : [char;5] = ['a', 'b', 'c', 'd', 'e'];
  let valor = car[1];
  println!("valor {:?}", valor);

  //modificar un array
  let mut car01 : [char;5] = ['a', 'b', 'c', 'd', 'e'];
  car01[1] = 'h';
  println!("valor {:?}", car01);
  
}

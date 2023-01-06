#[derive(Debug)]
struct Usuario {
    usuario: String,
    correo: String,
    veces_activo: u64,
    activo: bool,
}

//tupla como struct
#[derive(Debug)]
struct Color(i32, i32, i32);

//struct con impl
#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
  //funcion asociada
  fn rectangulo(ancho :u32, alto :u32) -> Rectangulo{
    Rectangulo{
      ancho, alto
    }
  }
  
    //metodo
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    //metodo
  fn area2(&self, rect2: &Rectangulo) -> u32 {
    println!("Nuestro rectangulo2 tiene las siguientes dimensiones: {:?}", rect2);
        self.ancho * self.alto
    
  }
}

fn struct() {
    let color = Color(3, 2, 1);
    println!("color: {:?}", color);

    let mut usuario = Usuario {
        usuario: String::from("wlopezob"),
        correo: String::from("w@h.c"),
        veces_activo: 3,
        activo: true,
    };
    println!("usuario: {:?}", usuario);
    println!("correo: {}", usuario.correo);
    usuario.veces_activo = 2;
    println!("veces activo: {}", usuario.veces_activo);

    let usuario2 = construir_usuario(String::from("wlopezob"), String::from("w@g.com"));
    println!("usuario2: {:?}", usuario2);

    let usuario3 = Usuario {
        usuario: String::from("usu"),
        correo: String::from("c@h.com"),
        ..usuario
    };
    println!("usuario3: {:?}", usuario3);

    let ancho: u32 = 10;
    let alto: u32 = 20;
    println!("El area es: {}", area(ancho, alto));

    //con tupla
    let di = (10, 200);
    println!("El area2 es: {}", area2(di));

    //creando una instancia
    let rect = Rectangulo {
        ancho: 20,
        alto: 30,
    };

    println!("Nuestro rectangulo es: {}", rect.area());

   let rect2 = Rectangulo {
        ancho: 100,
        alto: 100,
    };
    println!("Nuestro rectangulo2 es: {}", rect.area2(&rect2));

  //invocando a la funcion asociada
  let rect3 = Rectangulo::rectangulo(2,3);
    println!("Nuestro rectangulo3 es: {:?}", rect3);
  
}

fn construir_usuario(usuario: String, correo: String) -> Usuario {
    Usuario {
        usuario,
        correo,
        veces_activo: 1,
        activo: true,
    }
}

fn area(ancho: u32, alto: u32) -> u32 {
    ancho * alto
}

fn area2(dimensiones: (u32, u32)) -> u32 {
    dimensiones.0 * dimensiones.1
}

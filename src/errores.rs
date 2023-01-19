use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn leer_usuario_desde_fichero() -> Result<String, io::Error> {
    let mut f = File::open("hola.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

fn abrir_crear_archivo() {
    let f = File::open("hola.txt");
    let f = match f {
        Ok(fichero) => fichero,
        //Err(error) => panic!("Error abriendo el fichero!")
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hola.txt") {
                Ok(fichero_creado) => fichero_creado,
                Err(e) => panic!("Error creando el archivo {:?}", e),
            },
            other_error => panic!("Error al abrir el fichero {:?}", other_error),
        },
    };
}

fn errores() {
    abrir_crear_archivo();
    let texto = leer_usuario_desde_fichero().unwrap();
    println!("El contenido es {texto}");
}

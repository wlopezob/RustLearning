#[derive(Debug)]
enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Agosto,
    Setiembre,
    Octubre,
    Noviembre,
    Diciembre
}

#[derive(Debug)]
enum Tiempo {
    Segundo,
    Minuto,
    Dia(Mes)
}

fn main() {
    let maximo_configurado  = Some(7u8);
    match maximo_configurado {
        Some(maximo) => println!("El máximo configurado es {}", maximo),
        _ => ()
    }

    if let Some(maximo) = maximo_configurado {
        println!("El máximo configurado es {}", maximo);
    }

    //let tiempo = Tiempo::Segundo;
    let tiempo = Tiempo::Dia(Mes::Abril);
    let mut count = 0;
    if let Tiempo::Dia(mes) = tiempo {
        println!("Es un dia del mes: {:?}", mes);
    } else {
        count += 1;
    }
    println!("El valor del contador es {count}");
}
fn add_five<F>(func: F)
where
    F: Fn(i32),
{
    func(5)
}

fn add_five_mut<F>(mut func: F)
where
    F: FnMut(i32),
{
    func(5)
}

struct ClassicCars {
    make: &'static str,
    models: Vec<(&'static str, i32)>,
}

impl ClassicCars {
    fn smart_get<F>(&self, f: F) 
    where
        F: Fn(&Vec<(&'static str, i32)>) 
    {
        f(&self.models)
    }
}

fn main() {
    let clzr = |num| num + 1;
    let fn_hola = || {
        println!("hi");
    };
    println!("number is {}", clzr(1));
    fn_hola();

    let myvalue = 5;
    let clzr2 = |num| num + myvalue;
    println!("number is {}", clzr2(5));

    let clzr3 = |a, b| a + b;
    println!("number is {}", clzr3(1, 2));

    let clzr4 = |a, b| a + b;
    println!("result: {}", clzr4("hola".to_string(), " mundo"));

    let num1 = 6;
    add_five(|x| println!("{}", num1 + x));

    let mut num2 = 6;
    add_five_mut(|x| {
        num2 += x;
        println!("{}", num2);
    });

    let car_collection = vec![
        ("Thunderbird", 1960),
        ("Cobra", 1966),
        ("GT", 1967),
        ("Mustang", 1969),
    ];
    let ford_models = ClassicCars {
        make: "Ford",
        models: car_collection,
    };

    ford_models.smart_get(|x| {
        let res: Vec<_> = x.into_iter().filter(|x| x.1 > 1960).collect();
        println!("results {:?}", res);
        let res: Vec<_> = x.into_iter().map(|x| format!("{} {}", x.1, x.0)).collect();
        println!("results {:?}", res);

    });
}

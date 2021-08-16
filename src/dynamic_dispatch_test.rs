use std::any::{Any, TypeId};

struct Circle{
    radius: f64
}

struct Square{
    side: f64
}

trait Shape{
    fn area(&self) -> f64;
}

impl Shape for Circle{
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Shape for Square{
    fn area(&self) -> f64 {
        self.side * self.side
    }
}


pub fn call_dynamic_dispatch_test(){

    let shapes: [&Shape; 4] =[
        &Circle{radius : 1.0},
        &Circle{radius : 2.0},
        &Square{side : 3.0},
        &Square{side : 4.0},
    ];

    for (i, shape) in shapes.iter().enumerate(){ /*Es dynamic porque tengo un array que se "genero en tiempo de ejecucion" (en este caso no es cierto, pero podria ser un arrray que crece y va tomando diferentes cosas,
    entonces es necesario el dynamic dispatch porque no puedo asegurar que poronga va a haber adentro*/
        println!("Shape #{} has an area of {}", i , shape.area())
    }
}
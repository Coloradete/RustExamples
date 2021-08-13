
trait Animal{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self){
        println!("{}, cannot talk :C", self.name());
    }
}

struct Human{
    name: &'static str,
    age: u32
}

impl Animal for Human{
    fn create(name: &'static str) -> Human {
        Human{name, age: 0 }
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
}

struct Cat{
    name: &'static str,
    tails: u32
}

impl Animal for Cat{
    fn create(name: &'static str) -> Cat {
        Cat{name, tails: 0 }
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} prrs at you, and also says look at me I have {} tails!!",self.name, self.tails);
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{

    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {result += *x; }
        result
    }

}

pub fn call_traits(){

    // let jhon = Human{name:"Jhon", age: 55 }; //una mejor manera de crearlo seria la siguiente linea, el compilador elije que funcion create usar en base al tipo definido de la variable
    let jhon:Human = Animal::create("Jhon");
    jhon.talk();

    let mishi = Cat{name: "mishi", tails: 3};

    mishi.talk();

    let a = vec![1,2,3];
    println!("{}", a.sum());
}

//////////////////////////////
use std::fmt::Debug;

#[derive(Debug)]
struct Circle{
    radius : f64
}

trait Shape{
    fn area(&self) -> f64;
}

impl Shape for Circle{
    fn area(&self) -> f64{
        self.radius * self.radius * std::f64::consts::PI
    }
}

// fn print_info(shape: impl Shape + Debug)
// fn print_info<T: Shape + Debug> (shape: T, shape2: T)
fn print_info<T>(shape: T) where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("the area is: {}", shape.area());
}

pub fn call_traits_02(){

    let circle = Circle{radius: 2.5};

    print_info(circle);
}

use std::fmt::Debug;

#[derive(Debug)]
struct Person{
    name: String
}

impl Person{
    fn new<S>(name: S) -> Person where S: Into<String>
    {
        Person {name : name.into() }
    }
}


pub fn call_into_test(){

    let a = Person::new("nombre");

    println!("{:?}", a);
}
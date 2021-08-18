use std::rc::Rc;

struct Person{
    // name: String // Puedo definirlo como tipo Rc (reference counted)
    name: Rc<String>
}

impl Person{
    // fn new (name: String) -> Person
    fn new(name: Rc<String>) -> Person
    {
        Person{name}
    }

    fn greet(&self)
    {
        println!("Hi my name is {}", self.name);
    }
}

pub fn reference_counted_variable_call(){

    // let name ="Jhon".to_string();
    // let person = Person::new(name);

    let name = Rc::new("Jhon".to_string());
    println!("name has {} strong pointers", Rc::strong_count(&name));
    { //Este scope es solo para ver como cambian los pointers a la variable name cuando se limpian las variables que la clonaron, en este caso person

        let person = Person::new(name.clone()); //Le da el valor de name a person pero sin perder el ownership dentor de name
        person.greet();
        println!("name has {} strong pointers", Rc::strong_count(&name));
    }

    println!("name has {} strong pointers", Rc::strong_count(&name));

    println!("Name = {}", name); //Esto no funciona en el caso sin Rc porque perdi el ownership de la variable name al declarar Person::new(name), no puedo utilizarla despues.
}
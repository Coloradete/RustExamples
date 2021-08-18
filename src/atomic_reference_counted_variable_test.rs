use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, Arc};

struct Person
{
    name: Arc<String>,
    // state: Arc<String> //Tengo que definirla como Arc<Mutex> si quiero cambiarla en threads
    state : Arc<Mutex<String>>
}

impl Person{
    // fn new(name: Arc<String>, state: Arc<String>) -> Person
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person
    {
        Person{name, state}
    }

    fn great (&self){
        // self.state.clear();                         //Esto no funciona porque puede suceder que varios threads esten cambiando el valor de la variable al mismo tiempo y eso no es safe.
        // self.state.push_str("Excited");

        let mut state = self.state.lock().unwrap();// Mutex es abreviacion para mutual exclusion, esto lo que permite es que si está la misma variable en varios threads, no puede ser modificada en varios al mismo tiempo
        state.clear();                                              //Con el lock() estoy lockeando la variable para que solo este thread pueda modificarla
        state.push_str("excited");

        println!("Hi, my name is {}! AND IM {}!!!", self.name, state.as_str());
    }
}

pub fn call_arcv_test(){

    let name = Arc::new("Jhon".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));

    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.great();
    });

    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str()); //Toodo esto de lock unwrap y as_str es porque la defini como variable Mutex

    t.join().unwrap();
}
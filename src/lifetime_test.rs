
#[derive(Debug)]
struct Person{
    name: String
}

impl Person{

    //fn get_ref_name(&self) -> &String{ //Esto traería problemas en la linea 38 porque estoy asignando a z un valor dentro de un scope y luego tratando de obtenerlo fuera de el, en donde la variable ya no existe, se acabo su lifetime
                                        //Esto se llama lifetime illusion, en realidad Rust no usa la linea 9 al definir esta funcion, sino que usa la 11, con los lifetimes especificados
    fn get_ref_name<'a> (&'a self) -> &'a String{   //La recomendacion es no ponerlos porque se sobreentiende que son esos (es redundante ponerlo así), no te deja especificar funciones a referencias sin su lifetime
        &self.name
    }

}

#[derive(Debug)]
struct Company<'z>
{
    name: String,
    // ceo: &Person //Esto no puedo hacerlo porque tengo que definir cual es el lifetime de esta variable, si no hiciera esto entonces la variable podría volverse invalida en algun momento y Rust no permite referencias inválidas
    ceo: &'z Person //De esta manera digo que el lifetime de esta referencia va a ser igual al lifetime de la estructura Company entera (el lifetime de z)
}

pub fn call_lifetime_test()
{

    let boss = Person{ name: "Elon Musk".to_string()};
    let tesla = Company{name: "Tesla".to_string(), ceo: &boss};

    println!("{:?}", tesla);

    let mut z: &String;

    {
        let p = Person{name: String::from("Timmy")};
        z = p.get_ref_name();
        println!("{}",z);
    }

    // println!("{}",z); //Esta linea es la que rompe toodo, porque quiero acceder a la variable z cuya referencia esta asignada dentro del scope anterior y ya terminó




    let person_test = PersonTwo{name: "carlui"};
    person_test.talk();
}

struct PersonTwo<'a> { //Si especifico la estructura con una referencia en vez de con un valor entonces las tengo que especificar el tiempo de vida de la estructura y sus variables (la variable no puede vivir más que la estructura)
    name : &'a str
}
impl<'a> PersonTwo<'a>{ //la implementacion no puede vivir mas que la estructura, por eso se aclara el tiempo de vida como 'a, si no se aclara daría error
    fn talk(&self){
        println!("Hi my name is {}",self.name);
    }
}

trait Animal{
    fn create(name: &'static str) -> Self where Self:Sized; //No puede cambiar su tamaño, esto lo necesito para poder hacerle el Box

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

enum Creature{ //un enum pu8ede contener cualquier cosa, aca se usa cada uno de los casos del enum para representar la informacion de una estructura ya existente
    Human(Human),//el caso "Human" Usa la estructura del Human (Human) para almacenar la info
    Cat(Cat) //el caso Cat Usa la estructura del Cat (Cat) para almacenar la info
}

pub fn call_vectors_of_different_types(){
    let mut creatures = Vec::new();
    // creatures.push(Human{name:"Jhon", age: 0 }); //Esto no lo puedo hacer porque estoy metiendo dos tipos distintos dentro de un vector, por eso tengo qeu definir el enum de mas arriba
    // creatures.push(Cat{name:"Timmy", tails: 0 });

    creatures.push(Creature::Human( //Ahora si me deja porque lo que estoy formando es un vector de Creature (definido con el enum) y ta too' bien, cada enum usa su propio struct
        Human{name: "Jhon", age : 90}
    ));
    creatures.push(Creature::Cat(
        Cat{name: "Mishi", tails : 90}
    ));

    for c in creatures
    {
        match c{ //no puedo derecho decir c.talk() porque c es el enum, tengo que ver que typo es y ahi puedo llamar a la funcion que implementa
            Creature::Human(h) => h.talk(),
            Creature::Cat(d) => d.talk()
        }
    }

    let mut animals:Vec<Box<Animal>> = Vec::new(); //Creo un vector de objetos instanciados de animales y ahi si puedo hacerlo (ver mejor que es Box)
    animals.push(Box::new(Human{name:"Jhon",age : 45}));
    animals.push(Box::new(Cat{name:"Mishi",tails : 45}));

    for a in animals.iter(){
        a.talk();
    }
}
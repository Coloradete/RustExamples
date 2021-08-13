
struct Creature {
    name: String
}
impl Creature {
    fn new(name: &str) -> Creature
    {
        println!("{} enters the game", name);
        Creature{name: name.into()}
    }
}

impl Drop for Creature{
    fn drop(&mut self){
        println!("{} got cleaned from memory", self.name);
    }
}


pub fn call_drop_test(){

    let goblin = Creature::new("Jeff");

    // drop(goblin); //Esto si quisiera doprearla yo en algun sitio y no que se dropee instantaneamente al final del scope

    println!("Something happens");
}

trait Printable{
    fn format(&self) -> String;
}

impl Printable for i32{
    fn format(&self) -> String{
        format!("i32: {}", *self)
    }
}

impl Printable for String{
    fn format(&self) -> String{
        format!("String: {}", *self)
    }
}

fn print_it_static<T: Printable>(z: T) /*monomorphisation, en tiempo de compilacion tenemos una funcion print_it que toma i32 como entrada yu otra que toma un String,
 esto porque son nlas variables que usamos mas abajo. Se llama static dispatch porque la decicion de que implementacion del print_it hay que usar sucede en tiempo de
 compilacion y no de ejecucion, ya que conocemos el tipo de las variables al momento de llamar esa funcion haciendolo un Static Dispatch*/
{
    println!("{}", z.format());
}

fn print_it_dynamic (z: &Printable){ /*La diferencia con la anterior es que aquí no paso la variable entera, sino una referencia a ellas, por lo que pierdo la informacion
de si es un string o un i32 en tiempo de compilacion, entonces eso se debe descubrir en tiempo de ejecucion haciendolo un Dynamic Dispatch, mas caro de ejecutar y por lo tanto peor que el anterior*/
    println!("{}", z.format());
}


pub fn call_static_dispatch_test(){

    let a = 123;
    let b ="hello".to_string();

    println!("{}",a.format()); //Pero tambien podria definir una funcion (print_it) que haga exactamente esto
    println!("{}",b.format());

    // print_it_static(a);
    // print_it_static(b);

    print_it_dynamic(&a);
    print_it_dynamic(&b);
}


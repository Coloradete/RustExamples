
fn print_value (x: i32){
    println!("{}", &x);
}

fn increase(x: &mut i32){
    *x += 1;
}

// fn increase(mut x: i32){ //Si lo hiciera así entonces estoy pasando el valor de z a mut x y aumentando el de mut x, z no se ve modificada
//     x += 1;
// }

fn product(x: i32, y: i32) -> i32{
    x * y
}

pub fn call_functions(){
    print_value(33);

    let mut z = 1;
    
    increase(&mut z);
    print_value(z); //Aca no deberia perder el ownership de z? no debería ser print_value(x: &i32) pa que no pierda la referencia?

    increase(&mut z);
    print_value(z);
}
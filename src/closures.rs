
pub fn call_closures(){
    let plus_one = |x: i32| -> i32 {x + 1} ;

    let a = 6;

    println!("{} + 1 = {}", a, plus_one(a)); //no modifica el valor de a

    println!("{}", a);

    let plus_one_mut = |x: &mut i32| { *x += 1} ; //SI modifica el valor de b, fijate que no volves con un i32 como antes, volves con el valor de la variable de entrada

    let mut b = 6;
    println!("{}", b);

    plus_one_mut(&mut b);
    println!("{}", b);

    let mut two = 2;

    { //scope donde voy a usar el plus_two
        let plus_two = |x: i32|{
            let mut z = x;
            z += two;
            z
        };

        println!("plus_two(3):{}", plus_two(3));
    }

    let mut borrow_two = &mut two;

    // println!("plus_two(3):{}", plus_two(3)); //Esto me da error porque ahora two no existe, el ownership lo tiene borrow_two, buena practica seria poner un scope {} dentro del closure, para asegurarme de que se destruye fuera de el

    println!("{}", borrow_two);

    *borrow_two += 2;



}
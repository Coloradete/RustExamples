

pub fn call_borrowing_test(){


    let print_vector = |x: &Vec<i32>|
    {
        println!("{:?}", x);
    };

    let v = vec![1,2,3];
    print_vector(&v); //con el & en vez de estar dandole el ownership de v a print_vector, le estoy permitiendo tomarlo prestado, es una referencia a v y no v

    println!("v still has the ownership = {:?}",v);

    let mut a = 40;

    let b = &mut a; //aca b toma prestada la referencia muteable a a durante todoo el lifetime de b

    *b += 2;                //se modifica el valor almacenado en esa referencia (el valor en el heap si fuese un Box, pero en binario por ser una clase primitiva)

    println!("a = {:?}", a); //Si no tuviese la linea 24 podría hacer esto sin problemas porque b tendria un lifetime menor al de a, comono es así (porque b vuelve a usarse despues de a) me da un error.

    // println!("b = {:?}", b);

    /* Se puede tener mas de una referencia a un mismo valor almacenado en el heap (o a un recurso), pero solo puede existir UNA referencia MUTEABLE a ese recurso*/

    let c = &mut a; //si hago una segunda referencia muteable a a aca está todoo bien porque el tiempo de vida de b acaba en la linea 20, si subo esto mas arriba de esa linea tendré problemas de compilacion

}
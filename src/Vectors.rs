use std::process::id;

pub fn call_vectors(){
    let mut a = Vec::new();

    a.push(1);
    a.push(17);
    a.push(21);

    let mut idx:usize = 0; // el indice con el que accedo a los vectores o arrays tiene que ser si o si del tipo usize

    println!("a[{}] = {}", idx, a[idx]);

    idx = 6;
    // println!("a[{}] = {}", idx, a[idx]);//Esto falla porque estoy tratando de acceder a un elemnto inexistente en el vector, env ez de esto lo correcto sería hacer lo siguiente

    match a.get(idx)
    {
        Some(x) => println!("a[{}] = {}", idx, a[idx]),
        None => println!("error, no such element")
    }

    for x in &a{ println!("{}",x);}

    a.push(69);

    let last_elem = a.pop(); //remueve el ultimo valor de la lista a y lo mete en last_elem

    println!("last element is {:?}",last_elem);

    println!("a is {:?}",a);

    // let Some(value) = a.pop(); //Esto no se puede porque no esta considerado el caso en el que a.pop() devuelva un valor de tipo NONE, lo que si se podria hacer es lo siguiente

    if let Some(value) = a.pop(){
        println!("value {}", value);
    };

    //O tambien esto
    while let Some(x) = a.pop(){
        println!("{}",x);
    }
}
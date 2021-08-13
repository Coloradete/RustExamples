

pub fn call_iterators(){

    let mut vec = vec![1,2,3];

    for x in vec.iter_mut(){
        println!("{}",*x);
        *x += 3;
    }
    println!("{:?}", vec);

    for x in vec.iter_mut().rev(){
        println!("{}",*x);
    }

    let mut vec2 = vec![3,2,1];
    // let it = vec.into_iter();
    vec2.extend(vec);

    println!("{:?}", vec2);

    // println!("{:?}", vec); //esto no funciona, al llamar extend() se llama under the hood a into_iter() en vec y hace que este ya no exista
}
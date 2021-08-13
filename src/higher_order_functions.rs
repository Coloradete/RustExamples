
fn is_even(x: u32) -> bool{
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {

    move |y| y > limit
}


pub fn call_higher_order(){

    let limit = 500;

    let mut sum = 0;


    // let above_limit = |y| y > limit; //remplazo por higher-order function

    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i*i;

        // if isq > limit{          //lo puedo reemplazar por una closure, que a su vez se puede reemplazar por una higher-order function (greater_than)
        if above_limit(isq){
            break;
        }
        else if is_even(isq){
            sum += isq;
            println!("isq = {}", isq);
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)                                    //toma el rango de 0 a infinito
        .map(|x| x*x)               //los eleva al cuadrado
        .take_while(|&x| x < limit) // los usa mientras el valor x < limit
        .filter(|x: &u32| is_even(*x))  //los filtra segun la funcion de entrada is_even
        .fold(0, |sum,x| sum + x);      //hace la suma

    println!("hof sum = {}", sum2);
}
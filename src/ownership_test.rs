

pub fn call_ownership_test(){

    let v = vec![1,2,3];

    // let v2 = v;
    // println!("{:?}",v); //esto da error porque ahora v2 apunta al vec![1,2,3] y no v, no puedo tener dos variables apuntando a lo mismo, el ownership de ese vector lo tiene v2

    let foo = |v:Vec<i32>|();
    foo(v);

    // println!("{:?}",v); //esto da error porque ahora foo apunta al vec![1,2,3] y no v, no puedo tener dos variables apuntando a lo mismo, el ownership de ese vector lo tiene foo

    let u = 1;
    let u2 = u;

    println!("u = {}", u); /*sin embargo para primitivas si que funciona, ya que en vez de mover el ownership, se realiza una copia.
     Esto es porque el tipo i32 no tiene un pointer hacia informacion, son simplemente los 32 bits, al contrario del vector definido que se almacena en el heap */

    let d = Box::new(1);
    let d2 = d;
    // println!("d = {}", d);
    /*Este si falla porque ahora lo que estoy haciendo con Box es almacenar ese valor en el Heap y d ahora es un puntero hacia eso (no los bits que representan el numero), entonces al llamar
    d2 esta pasando lo mismo que antes, cambia el ownership de ese valor y no puedo printearlo porque ahora esta en d2, d queda invalidado*/

    let vec = vec![2,3,4];

    let print_vector = |x:Vec<i32>| -> Vec<i32>{
        println!("{:?}",x);
        x
    };

    let vec2 = print_vector(vec); //el ownership de vec lo tiene la funcion print_vector mientras hace cosas ccon ella y despues se lo da a vec2 al terminar.
    //esto no es tan comodo y por eso existe el concepto de borrowing (en otro file)

    println!("{:?}", vec2);
}
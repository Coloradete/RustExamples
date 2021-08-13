use std::ops::Add;

#[derive(Debug)]
struct Complex<T>{
    re: T,
    im: T
}

impl<T> Complex<T>{
    fn new(re: T, im: T) -> Complex<T>{
        Complex::<T>{re, im }
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T>  //Como estoy haciendo la operacion suma mas abajo, tengo que especificar que el tipo T tiene que soportarla, no podría ser otra estructura en donde la suma no esta definida por ejemplo
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

pub fn call_operator_overloading(){

    let a = Complex::new(1,3);
    let b = Complex::new(2,4);


    println!("{:?}",a);
    println!("{:?}",b);

    let c = a + b; //Esto no lo podria hacer si no hubiese definido la operacion sunma para esta estructura

    println!("{:?}",c);
}
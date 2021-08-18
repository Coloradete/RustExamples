use std::ops::{Add, AddAssign, Neg};
use std::process::Output;

#[derive(Debug, Eq, PartialEq)]
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

impl <T> AddAssign for Complex<T> where T: AddAssign<T>{

    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl <T> Neg for Complex<T> where T: Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex{
            re: -self.re,
            im: -self.im
        }
    }
}

// impl<T> PartialEq<Self> for Complex<T> where T: Eq {  //Puedo implementarlo yo o usar los derived como lo hice arriba, depende de que tan jodida sea la cosa (si los que vienen por default no me sirven)
//     fn eq(&self, rhs: &Self) -> bool {
//         if self.re == rhs.re{
//             if self.im == rhs.im{
//                 return true;
//             }
//         }
//         return false;
//     }
// }
//
// impl<T> Eq for Complex<T> where T: Eq {
// }


pub fn call_operator_overloading(){

    let a = Complex::new(1,3);
    let b = Complex::new(2,4);


    println!("{:?}",a);
    println!("{:?}",b);

    let mut c = a + b; //Esto no lo podria hacer si no hubiese definido la operacion sunma para esta estructura

    println!("{:?}",c);

    let d = Complex::new(5,-1);
    c += d; //Esto TAMPOCO lo podria hacer si no hubiese definido la operacion suma y asignacion para esta estructura

    println!("{:?}",c);

    c = -c;

    println!("{:?}",c);

    let e = Complex::new(5,-1);

    if c == c{
        println!("We are equal!");
    }
}
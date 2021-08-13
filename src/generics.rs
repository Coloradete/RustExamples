
struct Point<T>{
    x: T,
    y: T
}

struct CoolPoint<T,V>{
    x:T,
    y:V
}

struct Line<T>{ //Todos tienen que ser del mismo tipo
    start: Point<T>,
    end: Point<T>
}

pub fn call_generics(){

    let a = Point{x:0, y: 0};
    let b = Point{x:true, y: false};

    let c = CoolPoint{x: true, y: 0};
}
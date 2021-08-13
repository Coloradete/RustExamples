
enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8} //estas son "pistas" para los usuarios, ellos vana ver los nombres de las variables
}

pub (crate) fn try_color()
{

    let c: Color= Color::RgbColor(0,0,0);

    let c1: Color = Color::Blue;

    let c2:Color = Color::Cmyk {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 0
    };

    match c
    {
        Color::Red =>  println!("ROJO"),
        Color::Green =>  println!("VERDE"),
        Color::Blue =>  println!("AZUL"),
        Color::Cmyk {cyan:_,magenta:_,yellow:_,black:255 } | Color::RgbColor(0,0,0) => println!("Este es negro como tu alma"),
        Color::RgbColor(r, g, b) => println!("Este esta cheto, es {} {} {}",r,g,b),
        Color::Cmyk {cyan,magenta,yellow,black } => println!("Este esta cheto, es {} {} {} {}",cyan,magenta,yellow,black)
    }
}
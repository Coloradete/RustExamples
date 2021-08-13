
pub fn call_strings(){

    let s: &'static str = "hello there!"; //& (shared reference, cannot be changed) 'static(valid for the whole program lifetime str(type utf-8)

    for c in s.chars().rev(){
        println!("{}",c);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char);
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // let u: &str = &letters;

    // let z = letters + &letters; //esto no funciona, deberia ser de otra forma,por ejemplo:

    // let mut z = String::new(); //otra manera
    // z.push_str(&letters);
    // z.push_str(&letters);

    let z = format!("{}{}", letters,letters); //otra mas


    println!("{:?}",z);

    let mut abc = String::from("hello wolrd");
    let mut abc2 = "hello world".to_string();
}

pub fn call_format(){

    let hello = "hello";
    let rust = "Rust";

    let hello_rust = format!("{}, {}! usando format", hello, rust);

    println!("{}", hello_rust);



}
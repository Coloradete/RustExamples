
pub (crate) fn options_call(){
    let x= 3.0;
    let y= 1.5;

    let result = if y != 0.0 {Some(x/y)} else {None};

    match result {
        Some(z) => println!("{}/{}={}",x,y,z),
        None=>println!("error")
    }

    if let Some(z) = result{
        println!("not really sure que hace esto, perop z={}",z);
    }
}
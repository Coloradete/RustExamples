
pub fn do_pattern_matching()
{
    for x in 0..13
    {
        println!("{}: I have {} oranges",x, how_many(x));
    }

    let mut point = (0,4);

    match point
    {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}",y),
        (x,0) => println!("y axis, x ={}", x),
        (ref mut x,y) => println!("x = {}, y = {}", x,y),
        (_,y) => println!("x = ?, y = {}", y)
    }
}

fn how_many(x:i32) -> &'static str
{
    match x
    {
        0 => "no",
        1|2 => "one or two",
        12 => "a dozen",
        z @ 9...11 => "lots of", //11 esta incluido en el rango con el punto extra
        _ if(x%2 == 0) => "some",
        _ => "a few"
    }
}
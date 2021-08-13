
pub (crate) fn call_tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("sp= {:?}",sp);

    println!("{0}+{1}={2}, {0}*{1}={3}",x,y,sp.0,sp.1);

    let (a,b) = sp; //destructuring

    let sp2 = sum_and_product(7,8);
    let combined = (sp,sp2); //tupple de tupples

    println!("{:?}",combined);
    println!("last elem = {}",(combined.1).1);

    let((c,d),(e,f))=combined; //desestructurar tupple de tupples

    let foo = (true,42.0, -1i8); //multiples types

    let single = (42,) ; //tupple con unico valor

}

fn sum_and_product(x:i32, y:i32) -> (i32,i32)
{
    (x+y, x*y)
}
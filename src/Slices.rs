

// fn use_slice(slice: &[i32])
// {
//
// }

fn use_slice(slice: &mut[i32])
{
    slice[0] = 333;
}

pub (crate) fn call_slice(){

    let mut data =[1,2,3,4,5];

    println!("before {:?}",data);
    use_slice(&mut data);
    println!("after {:?}",data);
}
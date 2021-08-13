
union IntOrFloat
{
    i : i32,
    f: f32
}

pub (crate) fn call_int_or_float(){

    let mut iof = IntOrFloat{i : 123};
    iof.i = 234;
    iof.f = 1.1;

    let value = unsafe{iof.i};
    println!("{}",value);

    process_value(IntOrFloat{i:42});
    process_value(IntOrFloat{f:4.2});
}

fn process_value(value: IntOrFloat)
{
    unsafe{
        match value{
            IntOrFloat{i:42} =>{
                println!("Meaning of life value");
            }

            IntOrFloat{f} => {
                println!("{}",f);
            }
        }
    }
}
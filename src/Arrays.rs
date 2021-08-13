

pub (crate) fn create_array()
{
    let mut a : [i32;5] = [1,2,3,4,5];

    let b = [1;10];

    let c = [1u8; 10];

    let mtx:[[f32;3];2] = [
        [1.0,0.0,1.5],
        [0.0,2.0,3.5]
    ];

    println!("{:?}",mtx);

    let i= 1;
    let j = 2;
    println!("mtx[{}][{}]={}",i,j, mtx[i][j]);
}
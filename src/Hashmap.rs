use std::collections::HashMap;

pub fn call_hashmap()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);

    for (key,value) in &shapes{
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);

    println!("now square has value: {}", shapes["square".into()]);

    shapes.entry("circle".into()).or_insert(1);

    {
        let mut actual = shapes.entry("circle".into()).or_insert(0);
        *actual = 0;
    }

    println!("{:?}",shapes);
}
use std::collections::HashSet;

pub fn call_hashset(){
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}",greeks);

    greeks.insert("delta");
    println!("{:?}",greeks);

    let added_vega = greeks.insert("vega");

    if added_vega {
        println!("yay it was true");
    }

    if !greeks.contains("kappa"){
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");

    if removed {
        println!("we removed delta");
    }

    println!("{:?}",greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //subset
    println!("is {:?} a subset of {:?}? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint  =  no common elements
    
}
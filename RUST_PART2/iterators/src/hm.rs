use std::collections::HashMap;


pub fn hm () {
    let mut hm = HashMap::new();
    hm.insert("Rudra", 1);
    hm.insert("Rahul", 3);

    for (key,value) in hm.iter() {
        println!("{} , {}",key,value);
    }

    for (key,value) in hm.iter_mut() {
        *value += 10;
        println!("{} {}",key,value);
    }

    let owner = hm.into_iter();

    for (key,value) in owner {
        println!("{} {}",key,value);
    } 

}
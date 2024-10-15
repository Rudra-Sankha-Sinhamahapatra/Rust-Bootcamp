use std::collections::HashMap;

pub fn group_values_by_keys (vec:Vec<(String,i32)>) -> HashMap<String,i32> {
    let mut hm = HashMap::new();
    for (key,value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

pub fn lib() {
    let vec = vec![(String::from("Rudra"),20),(String::from("Rahul"),22)];
    let ans = group_values_by_keys(vec);

    println!("{:?}",ans);
} 
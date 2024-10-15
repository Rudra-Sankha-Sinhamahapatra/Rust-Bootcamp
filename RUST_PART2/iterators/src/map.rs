
pub fn map () {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.map(|x| x+1);

    for i in v1_iter2 {
        println!("{:?}",i);
    }

    println!("{:?}",v1);
}
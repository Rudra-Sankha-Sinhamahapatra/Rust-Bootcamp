
pub fn filter () {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.filter(|x| *x%2 ==0);

    for i in v1_iter2 {
        println!("{:?}",i);
    }

    println!("{:?}",v1);
}
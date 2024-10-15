pub fn opt () {
    let  v1 = vec ![1,2,3];
    let v1_iter = v1.iter();

    let sum : i32= v1_iter.sum(); // v1_iter ownership transferred

    println!("sum: {:?}",sum);

}
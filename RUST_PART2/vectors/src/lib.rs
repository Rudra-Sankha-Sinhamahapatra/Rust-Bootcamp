
pub fn lib (){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    println!("{:?}",vec);

    let input: &str = " 10 20 30 40 ";

    let input_trim = input.trim();
    println!("Input trim: {:?}", input_trim);  

    let split_whitespace: Vec<&str> = input_trim.split_whitespace().collect();
    println!("Split whitespace after trimming: {:?}", split_whitespace);

    let map: Vec<Result<i32, _>> = split_whitespace
        .iter()
        .map(|num| num.parse::<i32>())  
        .collect();

    println!("After mapping them from string to i32: {:?}", map); 

   
    let collect: Vec<i32> = map.into_iter()
        .map(|res| res.expect("Enter valid i32"))  
        .collect();

    println!("Collected as a vector: {:?}", collect);

    let input2 = String::from(" 10 20 30 40");
    let resvec : Vec<i32> = input2.trim().split_whitespace().map(|num| num.parse().expect("Enter valid numbers")).collect();
    println!("Even elements in the vector are : {:?}",resvec);
}
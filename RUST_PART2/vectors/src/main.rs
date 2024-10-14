use std::io::stdin;

fn main() {
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

    let mut input = String::new();

    println!("Enter vector elements (integers)");
    stdin().read_line(&mut input).expect("Failed to read");

    let input2 = String::from(" 10 20 30 40");
    let resvec : Vec<i32> = input2.trim().split_whitespace().map(|num| num.parse().expect("Enter valid numbers")).collect();
    println!("Even elements in the vector are : {:?}",resvec);

    let new_vec : Vec<i32> = input.trim().split_whitespace().map(|num| num.parse().expect("Enter valid numbers")).collect();
    println!("Even elements in the vector are : {:?}",even_filter(new_vec));

}

fn even_filter(vec:Vec<i32>) -> Vec<i32>{
    let mut vec1 = Vec::new();

    for val in vec {
        if val % 2 ==0 {
            vec1.push(val);
        }
    }

    return vec1;
}

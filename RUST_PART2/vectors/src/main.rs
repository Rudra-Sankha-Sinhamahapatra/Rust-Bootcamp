use std::io::stdin;
use vectors::lib;
fn main() {
    lib(); 

    let mut input = String::new();

    println!("Enter vector elements (integers)");
    stdin().read_line(&mut input).expect("Failed to read");

    let mut new_vec : Vec<i32> = input.trim().split_whitespace().map(|num| num.parse().expect("Enter valid numbers")).collect();
    println!("Even elements in the vector are : {:?}",even_filter(&new_vec));
    
    println!("{:?}",new_vec);
    println!("From even func: {:?} is {:?}",even(& mut new_vec),new_vec);

}

// approach 1
fn even (v:& mut Vec<i32>){
    for i in (0 ..v.len()).rev(){
        if v[i] % 2 !=0 {
            v.remove(i);
        } 
    }
}

//approach 2

fn even_filter(vec:&Vec<i32>) -> Vec<i32>{
    let mut vec1 = Vec::new();

    for val in vec {
        if val % 2 ==0 {
            vec1.push(*val);// de reference the number
        }
    }

    return vec1;
}

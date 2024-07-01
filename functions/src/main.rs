fn main(){
    let a=32;
    let b=20;
    let sum=do_sum(a,b);
   println!("Sum of {} and {} is {}",a,b,sum);
}

fn do_sum(a:i32,b:i32)->i32 {
    return a+b;
}

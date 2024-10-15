//write the logic to first filter all the odd values then double each value and create a new vector
pub fn assignment_1 () {
let vec = vec![1,2,3,4,5,6];
let ans = ans(vec);
println!("{:?}",ans);
}

pub fn ans (vec:Vec<i32>) -> Vec<i32> {
    let v1_iter = vec.iter().filter(|x| *x%2==1).map(|x| x*2);
   let mut vec2:Vec<i32>  = Vec::new();

   for i in v1_iter {
    vec2.push(i);
   }

   return vec2;

}
fn main() {
   let mut nums = vec![1,2,3];
   let iter = nums.iter();
   let mut v1 = nums.iter();

   for value in iter {
    println!("Got {:?}",value);
   }

   while let Some(val) =  v1.next() {
    println!("Got {:?}",val);
   }
   println!("{:?}",nums);

   let nums_iter = nums.iter_mut();
   
   for val in nums_iter {
    *val = *val + 1;
   }

   println!("{:?}",nums);
}

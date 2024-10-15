
pub fn lib() {
    let nums2 = vec![1,2,3];
   let mut v2 = nums2.into_iter();

   while let Some(val) = v2.next() {
    println!("into iter 1 :{:?}",val);
   }

   let nums3 = vec![1,2,3];
   let v3 = nums3.into_iter();

   for val in v3 {
    println!("into iter 2:{:?}",val);
   }

   // uses into iter under the hood 
   let n = vec![2,4];

   for val in n {
    println!("{:?}",val);
   }
}
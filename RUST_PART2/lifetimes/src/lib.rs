pub fn lib() {
let a ="Hello";
let b = "Hello";
println!("Calculating string length...");
println!("Result:{}",longest(a, b));
}

fn longest<'a>(a:&'a str,b:&'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }
   else if a.len() < b.len() {
    return b;
   }
   else {
    return "Both strings have same length";
   }
}
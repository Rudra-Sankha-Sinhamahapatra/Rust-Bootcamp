pub fn opt() {
    let a = String::from("Hello");
    let b = String::from("Hell");
    println!("Checking which string is the largest...");
    println!("Result: {}", largest(a, b));
}

fn largest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else if a.len() < b.len() {
        return b;
    } else {
        return String::from("Both Strings have same length");
    }
}

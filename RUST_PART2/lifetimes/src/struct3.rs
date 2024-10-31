use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn struct3() {
    let result = longest_with_an_announcement("hello", "world", "This is an announcement!");
    println!("The longest string is: {}", result);
}

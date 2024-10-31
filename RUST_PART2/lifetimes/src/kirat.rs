pub fn kirat() {
    let ans;

    let str1 = String::from("ysma");
    {
        let str2 = String::from("aa");
        ans = longest2(&str1, &str2);
        println!("{}",ans)
    }

    // println!("{}", ans); // str2 doesn't live long
}

fn longest2<'a>(a:&'a str,b:&'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

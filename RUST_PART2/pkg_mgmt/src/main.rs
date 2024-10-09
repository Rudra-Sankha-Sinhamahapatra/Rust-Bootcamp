use chrono::{Local,Utc};

fn main() {
 let now = Utc::now();
 println!("UTC:{}",now);

 let formatted = now.format("%Y -%m -%d %H:%M:%S");
 println!("Formatted date and time : {}",formatted);

 let local = Local::now();
 println!("Current date and time in local: {}",local);
}

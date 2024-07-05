use chrono::{Local,Utc};

fn main() {
   let utc=Utc::now();
   println!("Current Date and Time in Utc :{}",utc);

   let formatted=utc.format("%Y-%m-%d %H:%M:%S");
   println!("Formatted Date and Time:{}",formatted);

   let local=Local::now();
   println!("Current Date and Time in local:{}",local);
}

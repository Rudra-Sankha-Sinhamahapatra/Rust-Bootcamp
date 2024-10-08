// finding odd numbers between 2 numbers
fn main() {
    let ans = count_var(1, 4);
    println!("{}",ans);
}

fn count_var (num:u32,num1:u32) -> u32 {
    let mut count = 0;
            
    if num == num1 {
        return 0;
    }

    else if num < num1 {
        for i in (num+1) ..=(num1-1) {
          if i % 2 !=0 {
            count  += 1 ;
          }
        }
    }

    else if num > num1 {
        for i in (num1+1) ..=(num-1) {
          if i % 2 !=0 {
            count +=1 ;
          }
        }
    }

    return count;
}

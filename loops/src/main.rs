fn main() {
let i=9;
for p in 0..10{
println!("{}",p+i)
}
let sentence=String::from("Hello");
let first_word=get_first_word(sentence);
println!("First word is :{}",first_word);
}

fn get_first_word(sentence:String)->String{
    let mut ans=String::from("");
for char in sentence.chars(){
  ans.push_str(char.to_string().as_str());
    if char==' '{
        break;
    }
}
return ans;
}
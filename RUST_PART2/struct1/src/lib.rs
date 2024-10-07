
pub struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

pub fn rectarea (){
    let rect = Rect {
        height:26,
        width:50
    };

    print!("The area of the rectangle is {}\n",rect.area());
    print!("The perimeter of the rectangle is {}\n",rect.perimeter());
}
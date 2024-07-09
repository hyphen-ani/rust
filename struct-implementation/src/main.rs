use std::fmt::Debug;

struct Rectangle{
    width: u32,
    height: u32,
}

impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Rectangle : {}", self.width * self.height)
    }
}


fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    println!("{:?}", rect);
}

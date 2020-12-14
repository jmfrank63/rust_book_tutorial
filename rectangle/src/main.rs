
#[derive(Debug)]
struct Rectangle{
    width: usize,
    height: usize,
}

impl Rectangle{
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

}
fn main() {
    let rect = Rectangle { height: 20, width: 30};

    println!("{}", rect.area());

    let rect_other = Rectangle { height: 10, width: 15};
    println!("{}", rect.can_hold(&rect_other));

    println!("{}", rect_other.can_hold(&rect));

    println!("{:?}", rect);
    
}

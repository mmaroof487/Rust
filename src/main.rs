// fn main() {
// let mut s1= String::from("hello");
// update(&mut s1);
// println!("{}",s1);
// }

// fn update(s:&mut String){
//     s.push_str(" world");
// }

struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
         self.width * self.height
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());
}

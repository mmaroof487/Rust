// fn main() {
// let mut s1= String::from("hello");
// update(&mut s1);
// println!("{}",s1);
// }
// fn update(s:&mut String){
//     s.push_str(" world");
// }

// struct Rect {
//    width: u32,
//    height: u32,
// }
// impl Rect {
//     fn area(&self) -> u32 {
//          self.width * self.height
//     }
// }
// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     print!("The area of the rectangle is {}", rect.area());
// }

// fn main(){
//     let num = 12;
//     print!("The value of fibonacci of num is: {}",fib(num));
// }
// fn fib(n:u32) -> u32 {
//     if n <= 1 {
//         return n;
//     }
//     fib(n-1) + fib(n-2)
// }

// fn fib(n: u32) -> u32 {
//     let mut a: u32 = 0;
//     let mut b: u32 = 1;
//     for _ in 0..n {
//         let temp = a + b;
//         a = b;
//         b = temp;
//     }
//     a
// }

// fn main(){
//     let s11= String::from("hello");
//     print!("{}",len_word(&s11));
// }

// fn len_word(s: &String) -> usize {
//     // s.len()
//     s.chars().count()
// }

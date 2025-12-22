// =====================================================
// BEGINNER
// =====================================================

// -------------------- HELLO WORLD --------------------
// fn main() {
//     println!("Hello, Rust!");
// }

// -------------------- VARIABLES --------------------
// fn main() {
//     let x = 5;
//     let mut y = 10;
//     y += x;
//     println!("x = {}, y = {}", x, y);
// }

// -------------------- FUNCTIONS --------------------
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn main() {
//     println!("Sum = {}", add(3, 4));
// }

// -------------------- OWNERSHIP --------------------
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // println!("{}", s1); // moved
//     println!("{}", s2);
// }

// -------------------- BORROWING --------------------
// fn len(s: &String) -> usize {
//     s.len()
// }
// fn main() {
//     let s = String::from("hello");
//     println!("{}", len(&s));
// }

// -------------------- STRUCT --------------------
// struct Rect {
//     w: u32,
//     h: u32,
// }
// impl Rect {
//     fn area(&self) -> u32 {
//         self.w * self.h
//     }
// }
// fn main() {
//     let r = Rect { w: 10, h: 20 };
//     println!("Area = {}", r.area());
// }

// =====================================================
// INTERMEDIATE
// =====================================================

// -------------------- ENUM + MATCH --------------------
// enum Status {
//     Success,
//     Error(String),
// }
// fn main() {
//     let s = Status::Error(String::from("404"));
//     match s {
//         Status::Success => println!("OK"),
//         Status::Error(msg) => println!("Error: {}", msg),
//     }
// }

// -------------------- OPTION --------------------
// fn find_even(n: i32) -> Option<i32> {
//     if n % 2 == 0 { Some(n) } else { None }
// }
// fn main() {
//     match find_even(3) {
//         Some(v) => println!("Even: {}", v),
//         None => println!("Odd"),
//     }
// }

// -------------------- RESULT --------------------
// fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
//     if b == 0 { Err("divide by zero") } else { Ok(a / b) }
// }
// fn main() {
//     println!("{:?}", divide(10, 2));
// }

// -------------------- ITERATORS --------------------
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     let sum: i32 = v.iter().filter(|x| *x % 2 == 0).sum();
//     println!("Sum of evens: {}", sum);
// }

// -------------------- CLOSURES --------------------
// fn main() {
//     let factor = 2;
//     let mul = |x| x * factor;
//     println!("{}", mul(5));
// }

// -------------------- LIFETIMES --------------------
// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() { a } else { b }
// }
// fn main() {
//     let s1 = "hello";
//     let s2 = "world!!!";
//     println!("{}", longest(s1, s2));
// }

// =====================================================
// ADVANCED
// =====================================================

// -------------------- TRAITS --------------------
// trait Shape {
//     fn area(&self) -> u32;
// }
// struct Square {
//     side: u32,
// }
// impl Shape for Square {
//     fn area(&self) -> u32 {
//         self.side * self.side
//     }
// }
// fn main() {
//     let s = Square { side: 4 };
//     println!("Area = {}", s.area());
// }

// -------------------- GENERICS --------------------
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut max = list[0];
//     for &item in list {
//         if item > max {
//             max = item;
//         }
//     }
//     max
// }
// fn main() {
//     let nums = vec![1, 3, 2, 5, 4];
//     println!("{}", largest(&nums));
// }

// -------------------- TRAIT OBJECTS --------------------
// trait Draw {
//     fn draw(&self);
// }
// struct Circle;
// impl Draw for Circle {
//     fn draw(&self) {
//         println!("Drawing circle");
//     }
// }
// fn main() {
//     let shapes: Vec<Box<dyn Draw>> = vec![Box::new(Circle)];
//     for s in shapes {
//         s.draw();
//     }
// }

// -------------------- SMART POINTER: BOX --------------------
// fn main() {
//     let b = Box::new(5);
//     println!("{}", b);
// }

// -------------------- RC (REFERENCE COUNTING) --------------------
// use std::rc::Rc;
// fn main() {
//     let a = Rc::new(10);
//     let b = Rc::clone(&a);
//     println!("a={}, b={}", a, b);
// }

// -------------------- REFCELL (INTERIOR MUTABILITY) --------------------
// use std::cell::RefCell;
// fn main() {
//     let data = RefCell::new(5);
//     *data.borrow_mut() += 1;
//     println!("{}", data.borrow());
// }

// -------------------- THREADS --------------------
// use std::thread;
// fn main() {
//     let handle = thread::spawn(|| {
//         println!("Hello from thread");
//     });
//     handle.join().unwrap();
// }

// -------------------- CHANNELS --------------------
// use std::sync::mpsc;
// use std::thread;
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         tx.send("hello").unwrap();
//     });
//     println!("{}", rx.recv().unwrap());
// }

// -------------------- ASYNC / AWAIT --------------------
// use tokio::time::{sleep, Duration};
// #[tokio::main]
// async fn main() {
//     sleep(Duration::from_secs(1)).await;
//     println!("Done");
// }

// -------------------- MANUAL FUTURE --------------------
// use std::future::Future;
// use std::pin::Pin;
// use std::task::{Context, Poll};
// struct MyFuture {
//     done: bool,
// }
// impl Future for MyFuture {
//     type Output = ();
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
//         if self.done {
//             Poll::Ready(())
//         } else {
//             self.done = true;
//             cx.waker().wake_by_ref();
//             Poll::Pending
//         }
//     }
// }

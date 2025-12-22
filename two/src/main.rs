pub trait Iterator{
    type Item=u32;
    fn next(&mut self)->Option<Self::Item>{
        if self.count<5{
            self.count+=1;
            Some(self.count)
        }else{
            None
        }
    };
}
struct Counter{
    count:i32
}
impl Counter{
    fn new(count:i32)->Self{
        Self{count=0}
    }
}


fn main() {
    let mut counter=Counter::new(0);
    while let Some(num)=counter.next(){
        println!("{}",num);
    }
}

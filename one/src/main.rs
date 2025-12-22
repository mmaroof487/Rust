pub trait Course{
    fn get_overview(&self);
}

struct Workshop{
    title: String,
    instructor: String,
    duration: u32
}
struct Seminar{
    title: String,
    speaker: String,
    duration: u32
}
struct Lecture{
    title: String,
    lecturer: String,
    duration: u32
}
impl Course for Workshop{
    fn get_overview(&self){
        println!("Workshop: {} by {}, Duration: {} hours",self.title,self.instructor,self.duration);
    }
}
impl Course for Seminar{
    fn get_overview(&self){
        println!("Seminar: {} by {}, Duration: {} hours",self.title,self.speaker,self.duration);
    }
}
fn print_course_overview<T:Course>(course: T){
    course.get_overview();
}
fn main() {
    let workshop = Workshop {
        title: String::from("Rust Programming"),
        instructor: String::from("Alice"),
        duration: 5,
    };
    let seminar = Seminar {
        title: String::from("Advanced Rust"),
        speaker: String::from("Bob"),
        duration: 3,
    };
    workshop.get_overview();
    seminar.get_overview();
    print_course_overview(workshop);
    print_course_overview(seminar);
}

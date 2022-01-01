#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    score: u8,
}

impl Student {
    pub fn new(name: String, score: u8) -> Self{
        Student {
            name,score
        }
    }
    
    pub fn get_score(&self) -> u8{
        self.score
    }

    pub fn set_score(&mut self, score: u8) {
        self.score = score;
    }
}

fn main(){

    let mut student = Student::new("muniz".to_string(), 98);

    let score = student.get_score();

    println!("muniz 的成绩是{}", score);

    student.set_score(100);
    let score = student.get_score();

    println!("muniz 的成绩是{}", score);


}
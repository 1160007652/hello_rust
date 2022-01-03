/**
 * 特征系统，trait 如同，传统语言中的接口抽象方法
 * 
 */

 use std::fmt::{Display, Formatter, Result};

 trait Geometry {
     fn area(&self) -> f32;
     fn perimeter(&self) -> f32;
 }

 struct Rectangle {
     width: f32,
     height: f32,
 }

 // trait 的定义与实现
 impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
 }

 // 格式化结构体数据 Display
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle: ({}, {})", self.width, self.height)
    }
}

 struct Circle {
     radius: f32,
 }

 // trait 的定义与实现
 impl Geometry for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
 }


 // 格式化结构体数据 Display
 impl Display for Circle {
     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
         write!(f, "Circle: ({})", self.radius)
     }
 }
 
 // trait 作为参数
 fn print_trait(geomerty: impl Geometry + Display) {
    println!("{}", geomerty);
    println!("矩型面积是{}", geomerty.area());
    println!("矩型周长是{}", geomerty.perimeter());
 }



 fn main(){
    let rect = Rectangle {width: 2.0 , height: 3.0};
    print_trait(rect);

    let circle = Circle {radius: 3.0};
    print_trait(circle);

 }

/**
 * 
 * 学习 泛型 知识
 * 
 */

struct Rectangle<T> {
    width: T,
    height: T
}

impl<T> Rectangle<T> {
    pub fn width(&self) -> &T {
        &self.width
    }
    pub fn height(&self) -> &T {
        &self.height
    }
}

impl Rectangle<i32> {
    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn option_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    if a.is_none() && b.is_none() {None} 
    else if a.is_none() {b}
    else if b.is_none() {a}
    else {
        Some(a.unwrap() + b.unwrap())
    }
}

fn option_paint(opt: Option<i32>) {
    match opt {
        Some(result) => println!("Option: {}", result),
        _ => println!("Option None"),
    }
}

fn foo<T>(x: T) -> T {
    x
}

fn main(){

    // 泛型变量 
    let mut arr: Vec<i32> = vec![1,2,3];
    arr.push(1);
    // arr.push("hello");  // 不能将字面量 &str 类型传入 i32 类型中


    // 泛型结构体
    let rect = Rectangle{width: 2, height: 4};
    println!(" 面积 is {}", rect.area());
    println!(" width is {}", rect.width());
    println!(" height is {}", rect.height());



    // 泛型枚举
    let result = option_add(Some(3),Some(5));
    option_paint(result);

    // 泛型函数
    let result = foo(6);
    println!("泛型函数{}", result);

}
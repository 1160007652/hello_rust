fn main() {

    let age = 26;

    if age > 18 {
        println!("已成年");
    }

    if age > 20 {
        println!("今年 {} 岁了", age);
    } else {
        println!("今年 {} 岁了, 没过20岁", age);
    }

    let age = 8;

    if age > 10 {
        println!("大于10");
    } else if age > 5{
        println!("大于5");
    } else {
        println!("小于5");
    }


}
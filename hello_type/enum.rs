
/*
 * 学习 枚举类型 的知识
 * 
 * - 不带类型参数的枚举值， 常规枚举
 * 
 * enum Color {
 *    Red,
 *    Yellow,
 *    Blue
 * }
 * 
 * - 带类型参数的枚举值
 * 
 * enum AppleColor {
 *  Red(String),
 *  Yellow(String)
 * }
 * 
 */

#[derive(Debug)]
enum HatColor {
    Red,
    Yellow,
    Blue
}

#[derive(Debug)]
enum AppleColor {
    Red(String),
    Yellow(String)
}

fn main(){

    let hat_color = HatColor::Blue;
    println!("这是一顶{:?}的帽子", hat_color);

    let apple_color = AppleColor::Red("红色的大苹果".to_string());
    println!("这是一个{:?}", apple_color);
    println!("这是一个{:?}", AppleColor::Yellow("黄香蕉苹果耶好吃".to_string()));

    println!("这是一顶{:?}的帽子", HatColor::Red);
    println!("这是一顶{:?}的帽子", HatColor::Blue);
    println!("这是一顶{:?}的帽子", HatColor::Yellow);

}
 
 
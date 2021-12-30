
/*
 * 学习 结构体类型 的知识
 * 
 * - 常规结构体
 * 
 * struct Person {
 *    age: i8,
 *    name: string
 * }
 * 
 * - 元组结构体
 * 
 * struct Color (String, String, String);
 * 
 */



struct Color (String, String, String);

struct Person {
    age: i8,
    name: String,
    hat: Color
}

fn main(){

    let hat_color = Color("white".to_string(), "blue".to_string(), "yellow".to_string());

    let mut muniz = Person {
        age: 26,
        name: "muniz".to_string(),
        hat: hat_color
    };


    println!("{} 今年{}岁, 带着一顶<{}>的帽子，在十字路口走失", muniz.name, muniz.age, muniz.hat.0);

    muniz.age = 102;

    println!("{} {}岁 寿辰，儿孙满堂，腰缠万贯", muniz.name, muniz.age);

}
 
 
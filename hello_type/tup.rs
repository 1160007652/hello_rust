
/*
 * 学习 元组类型 的知识
 * 
 * 元组类型：使用小括号 (1,2,3) 把元素包裹在一起，用逗号隔开
 * 
 * 支持使用 () 解构元组
 */


fn main(){

    let tup1: (i8, f32, bool) = (127,2.01, false);

    let tup2 = (7.7, (true, 10));

    println!("{} , {} , {} , {}", tup1.0, tup1.1, tup1.2, (tup2.1).1);


    let (x,y,z) = tup1;
    println!("{} , {} , {}", x,y,z);

    let (_, a) = tup2;

    println!("{} , {}", a.0, a.1);

}
 
 
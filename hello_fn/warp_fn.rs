

// 高阶函数， 

// 定义类型
type MathOp = fn(a:i32, b:i32) -> i32;


// 函数作为参数进行传递
fn math(op: MathOp, a:i32, b:i32) -> i32 {
    op(a,b)
}

// 函数作为返回值
fn math_op(flag: &str) -> MathOp {
   match flag {
       "add" => add,
       _=> subtract,
   }
}

fn add(a:i32, b:i32) -> i32 {
    a+b
}

fn subtract(a:i32, b:i32) -> i32 {
    a-b
}

fn main() {

    let result = math(add, 1, 2);
    println!("1+2={}",result);

    let result = math_op("add")(1,2);
    println!("1+2={}",result);

}
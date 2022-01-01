fn add(a: i32, b:i32) -> i32 {
    a+b
}

fn main(){

    let amount = {
        let price = 10;
        price / 5
    };

   let sum = add(1,amount);
   println!("1+{}={}",amount,sum);


   // 闭包函数

   let add_one = |a:i32, b:i32| -> i32 {a+b};

   println!("闭包函数结果 = {}", add_one(1,2));
}
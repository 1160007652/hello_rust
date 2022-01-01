// use std::vec;

fn main() {

    let arr = [1,2,3,4,5,6,7,8,9];

    let mut arr_iter = arr.iter();
    println!("{:?}", arr_iter.next());



    // sum 迭代器 求和
    let sum: i32 = arr_iter.clone().sum();
    println!("iter sum {}", sum);

    // any 查找迭代器中是否有存在的数据
    let result = arr_iter.clone().any(|&x| x==5);
    println!("iter any {}", result);

    // collect 转换到数组、map
    let result: Vec<i32> = arr_iter.clone().map(|x| x+1).collect();
    println!("iter collect {:?}", result);
  
    // take 生成新的迭代器数量大小的，迭代器
    let result = arr_iter.clone().take(3);
    println!("iter take:3 = {:?}", result);
    for item in result {
        println!("tahe:{}",item);
    }

    // rev 反转迭代器
    let result = arr_iter.clone().rev();
    println!("iter rev = {:?}", result);
    for item in result {
        println!("tahe:{}",item);
    }

    // zip 压缩迭代器，
    let v1 = [1,2,3];
    let v2 = [2,3,4];
    let v3 = [3,4,5];


    let result: Vec<i32> = v1.iter().zip(v2.iter().zip(v3.iter())).map(|(a, (b, c))| a+b+c).collect();
    println!("zip = {:?}", result);

    let result: Vec<i32> = v1.iter().zip(v2.iter()).map(|(a,b)| a+b).filter(|x| x % 3 == 0).collect();

    println!("iter filter > 5 = {:?}", result);

}
fn main() {

    let arr = vec![1,2,3,4,5];

    for item in arr {
        println!("结果{}", item);
    }

    for item in 1..3 {
        println!("结果{}", item);
    }

    for item in 1..=3 {
        println!("结果{}", item);
    }
}

/**
 * 
 * for 循环语句 也支持使用 brak continue
 * 
 */
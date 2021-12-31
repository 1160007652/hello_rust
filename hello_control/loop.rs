fn main() {
    let mut count = 0;

    // 循环无返回值
    loop {
        count += 1;

        println!("当前循环次数 {}", count);

        if count > 3 {
            break;
        }
    }


    // 循环有返回值
    let counter = loop {
        count += 1;

        println!("当前循环次数 {}", count);

        if count > 6 {
            break count;
        }
    };

    println!("循环{}次的结果是{}", count, counter);
}

/**
 * 
 * loop 循环知识点
 * 
 * - 结束循环的条件，遇到 break
 * 
 * - 支持返回值
 * 
 */
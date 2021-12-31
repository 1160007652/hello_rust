fn main() {

    let mut count = 0;

    while count < 3 {
        count += 1;

        println!("循环第{}次", count);

    }


    while count < 7 {
        count += 1;

        if count == 4 {
            continue; // 跳出当前循环
        }

        if count == 6 {
            break; // 结束循环
        }

        println!("循环第{}次", count);

    }


}
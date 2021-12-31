fn main() {

    let mut num = 99;

    match num {
        0..=20 => println!("输入的数字在 0 ～ 20 之间"),
        21..=60 => println!("输入的数字在 20 ～ 60 之间"),
        _ => println!("输入的数字在 60 ～ 100 之间"),
    }

    // 在使用 if let 时，等号左边不能放 变量
    if let 99 = num {
        println!("num = {}", num);
    }


    while let 99 = num {
        num += 1;
        println!("num != {}", num);
    }

}
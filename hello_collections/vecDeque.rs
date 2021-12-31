
use std::collections::VecDeque;

fn main(){

    let mut v: VecDeque<i32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_front(0);
    v.push_front(-1);
    println!("VecDeque::new() 双端队列 动态可变长数组 = {:?}", v);

}
  
  /*
   * 动态可变长数组, Vec 双端队列，同时具有 堆栈的 数据结构。
   * 
   * 定义方式：
   * 
   *  - let mut v: VecDeque<i32> = VecDeque::new();
   * 
   *  - let mut v = VecDeque::with_capacity(10);
   * 
   *  - 无宏创建方法，如 VecDeque![] , 不存在此方法
   *    
   */
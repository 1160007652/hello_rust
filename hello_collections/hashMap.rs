
use std::collections::HashMap;

fn main(){

    let mut map: HashMap<&str, i32> = HashMap::new();

    let age = map.insert("muniz", 26);
    println!("首次插入，无该键，返回 None  = {:?}", age);

    let age = map.insert("muniz", 27);
    println!("非首次插入，有该键，触发更新 返回旧值（上一次的值） = {:?}", age);

    println!("访问 hashMap 数据 = {}, {:?}", map["muniz"], map.get("muniz"));

    let name = map.remove("muniz");
    println!("删除键，以及对应的数据 = {:?}", name);



}
  
  /*
   * 哈希表, HashMap, 基于键值对的数据结构存储方式。
   * 
   * 定义方式：
   * 
   *  - let mut map: HashMap<&str, i32> = HashMap::new();
   * 
   *  - let mut map: HashMap<&str, i32> = HashMap::with_capacity(10); // 创建指定起始容量的 HashMap
   * 
   *  - 无宏创建方法，如 VecDeque![] , 不存在此方法
   *    
   */
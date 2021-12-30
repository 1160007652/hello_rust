

fn main(){

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("Vec::new() 动态可变长数组 = {:?}", v);


    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(3);
    v.push(4);
    println!("Vec::with_capacity(10) 动态可变长数组 = {:?}", v);

    let v = vec![1,2,3]; 
    println!("vec![1,2,3] 动态可变长数组 = {:?}", v);

    let v = vec![1;5];
    println!("vec![1;5] 动态可变长数组 = {:?}", v);

  
  }
  
  /*
   * 动态可变长数组, Vec
   * 
   * 定义方式：
   * 
   *  - let mut v: Vec<i32> = Vec::new();  // 创建动态可变长数组
   *    
   *  - let mut v: Vec<i32> = Vec::with_capacity(10); // 创建容量起步最小空间大小的可变长数组
   * 
   *  - let mut v: Vec<i32> = vec![]; // 使用！宏来创建 可变长数组
   * 
   *  - let mut v: = vec![1,2,3]; // 创建初始化数据为 1，2，3 的可变长数组
   * 
   *  - let mut v: = vec![1;5]; // 创建5个元素，元素值都是1 的可变长数组
   * 
   */
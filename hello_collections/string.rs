
fn main(){

  let mut s = "hello, ".to_string();


  // 追加操作
  s.push('W'); // 只能追加 char 类型 字符

  s.push_str("ord"); // 可以追加字符串

  println!("字符串 push 的结果 = {}", s);


  // 插入操作
  s.insert(7, 'R'); // 只能追加 char 类型 字符

  s.insert_str(8, "ust "); // 可以追加字符串

  println!("字符串 insert 的结果 = {}", s);


  // 拼接操作
  let s1 = "hello".to_string();
  let s2 = " word";
  let s3 = s1 + s2;
  println!("+ 字符串结果 = {}", s3);

  let s4 = format!("{} - {}",  s2 ,s3);
  println!("format! 字符串结果 = {}", s4);

  // 替换方法
  let mut s = String::from("hello , word word word");

  s = s.replace("word", "rust");
  println!("replace 结果 {}",s);

  s = s.replacen("rust", "word", 2);
  println!("replace 结果 {}",s);


  // 删除操作
  let mut s = String::from("hello, word rust");

  println!("pop 删除字符串最后一个字符，并返回删除的字符 {:?}", s.pop());
  println!("remove 删除字符串指定位置的字符，并返回删除的字符 {:?}", s.remove(5));

  s.truncate(5);
  println!("truncate 删除字符串指定位置到结尾的全部字符, 无返回值 {:?}", s);

  s.clear();
  println!("clear 清空，删除全部字符串 {:?}", s);


  // 字符长度
  let s = "L😯we 老虎".to_string();
  println!("{:?} 长度是 {}", s, s.len());
  
  let s = "L".to_string();
  println!("{:?} 长度是 {}", s, s.len());

  let s = "😯".to_string();
  println!("{:?} 长度是 {}", s, s.len());

  let s = "虎".to_string();
  println!("{:?} 长度是 {}", s, s.len());


  
  // 迭代字符串
  let s = "L😯we 老虎".to_string();
  println!("字节 bytes {:?}", s.bytes());
  println!("字符 chars {:?}", s.chars());


}
  
  /*
   * 字符串, String, 特殊的容器类型
   * 
   * 定义方式：
   * 
   *  - 固定字面量的字符串
   * 
   *    let s = "hello word";
   *  
   *    let str = String::from("hello word").as_str(); // 对象转字面量
   * 
   *  - 可变长度的字符串对象
   * 
   *    let mut s = String::new();
   * 
   *    let mut s = String::from("hello word");
   * 
   *    let mut s = "hello word".to_string(); // 字面量转对象
   * 
   * 知识点：
   * 
   *  [直连拼接操作]
   *  - 字符串对象与字面量 ，通过 + 做拼接时，字符串对象在最左边，字面量在 字符串对象后面进行加法拼接
   *  - 字符串对象，实现了 Deref trait ， 执行 + 操作时，会自动解引用到 &str 类型
   *  - 可以使用宏指令进行拼接操作，format!()
   * 
   *  [删除操作]
   *  - truncate(5), 删除起始位置5后面的全部字符
   *  - clear 删除全部字符，清空
   * 
   *  [获取字符长度]
   *  - len() 方法, 获取的是以字节为单位的字符串长度
   * 
   *  [迭代字符串]
   *  - 可以按照 字节 处理，使用 bytes() 方法，返回可迭代的字节类型迭代器
   *  - 可以按照 字符 处理，使用chars() 方法，返回可迭代的字符类型迭代器
   *    
   */
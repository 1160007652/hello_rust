
// 学习 数组类型 的知识

fn main(){

   let arr1:[i32; 5] = [1,2,3,4,5];

   println!("arr1[0] = {}", arr1[0]);

   let arr2 = [1,2,3,4,5];

   println!("arr2[0] = {}", arr2[0]);

   let arr2 = [1; 5];

   println!("arr2[0] = {}", arr2[0]);
 
 }
 
 /*
  * 数组类型：声明数组类型 [T; N] , T表示数组类型，N表示数组长度
  *  
  *  - let arr:[i32; 5] = [1,2,3,4,5]; 
  *    
  *  - let arr2 = [1; 5]; // 等价于 [1,1,1,1,1]
  * 
  */
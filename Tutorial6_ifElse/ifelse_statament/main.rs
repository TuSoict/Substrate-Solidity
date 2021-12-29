fn main() {
   let n = 123;

   if n>= 30 {
       println!("n >= 30 value: {}", n);
   } 
// khi thoa man 1 th thi no dung het cac cua lenh ve sau
   else if n > 100 {
        println!("n >= 100 value: {}", n);  
   }
   
   else {
       println!("n >= 30 value: {}", n);
       println!("n <> 30 : {}", n);
   }

   if n == 23 {
    println!("ok fine {}", n);
   }
}

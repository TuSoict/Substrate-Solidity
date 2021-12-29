// let mut a = 10; ==> lỗi

mod test;

const A: u32 = 10;

// Bai #11 Constants

const MAXIMUM_NUMBER: u8 = 24;

fn main() {
    /*
    Let cos thể không cần khai báo kiểu dữ liệu lúc đầu nhưng const bắt buộc phải khai báo kiểu dữ liệu và giá trị cho nó 
    và tên biến khai báo với const thì nên viết hoa nếu không khi compile sẽ có warning

    let có thể dùng để hứng giá trị của biến let x = a + b; Rust là ngôn ngữ lập trình kiểu dynamic nên khi compile sẽ biết đc kiểu dữ liệu của biến

    const có thể khai báo là global bên ngoài hàm main còn let thì không

    const không thể gán lại giá trị cho biến, không thể sử dụng từ khóa mut cho const const mut a: u32 = 10;  sẽ gặp lỗi 
    nhưng let có thể gán lại giá trị cho biến bằng từ khóa mut
    */
    const Y: i32 = 45;
    let mut x = 10;

    println!("This is value of x: {}", x);
    println!("This is value of y: {}", Y);

    x = 20;
    println!("This is value of x: {}", x);

  //  println!("This is value of a: {}", a);

     println!("This is value of A: {}", A);

     // Bai 11 ve Constant

     for i in 1..MAXIMUM_NUMBER {
       println!("{}", i);
     }

     //Bai 12 -  tuples

     let tup = (1, "Rust", false, (34,"Rust Developer", 7));

     println!("This is value of tuple: {}", tup.1);
     println!("This is value of tuple: {}", (tup.3).1);

     let (a, b, c, d) = tup;

     println!("Rest in tup {}", a);
     println!("Rest in tup {}", d.2);

     let (a1, ..) = tup;
     println!("Lay gia tri phan tu dau {}", a1);

     // goi 1 ham tu 1 fle khac

     test::test_fn();
     let val = test::return_value(5, -98);

     println!("Gia tri ham tra ve la:  {}", val);
}

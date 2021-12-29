//Bài 11: Khai báo và sử dụng Const
//Const là biến khai báo global và không thể thay đổi giá trị
// Ví dụ....................................................................................

// const MAXA: u8 = 10;

// fn main() {
//     for a in 1..=MAXA {
//         println!("Giá trị là {}",a);
//     }


// }

// Hết bài 11..........................................................................

// Bài 12: Tuple type
// Định nghĩa: Tuple thuộc về kiểu Compound types trong RUST (ngoài ra còn có kiểu Array)
// Là 1 nhóm các số (scalar type) vào với nhau. Tuple có độ dài cố định sau khi khai báo
// Cách khai báo: let tup = (1, 5.4, false, "Tuple")
// Có thể định nghĩa kiểu giá trị luôn: let tup: (i32, f32, bool, &str) = (1, 5.4, false, "Tuple")
// Cách lấy dữ liệu trong Tuple theo ví dụ......................................................
// use std::io;
// fn main() {
//     let tup = ("Ngày", 27, 3.6, true);
//     let (a, b, c, d) = tup;
//     println!("Giá trị a là {}, b là {}, c là {} và d là {}", a, b, c, d);
//     //Hoặc truy xuất giá trị theo thứ tự index 0..n. Ví dụ
//     println!("Giá trị đầu tiên là {}, giá trị cuối cùng là {}", tup.0, tup.3);

//     println!("Mời bạn nhập số từ 0 tới 3");
//     let mut index = String::new();
//     io::stdin()
//     .read_line(&mut index)
//     .expect("Không đọc được giá trị vừa nhập, vui lòng nhập 1 số nguyên");
   
//     let idx: u32 = 1;
//     let element = tup.idx;
//     println!("Giá trị bạn muốn tìm có phải là {}", element);

// }


// Mở rộng: Khai báo kiểu tuple phải liệt kê hết ra, còn kiểu array thì
// let arr = [3; 5]...bằng với [3, 3, 3, 3, 3] nhưng các thành phần trong array phải cùng 1 kiểu dữ liệu
// Có thể truy cập array bằng arr[index]; và dùng index để làm variable truy xuất dữ liệu của array
//Ví dụ...........................................................
// use std::io;

// fn main() {
//     let arr = ["Ngày", "Tháng", "Năm", "Linh vật"];
//     let [a, b, c, d] = arr;
//     println!("Giá trị a là {}, b là {}, c là {} và d là {}", a, b, c, d);
//     //Hoặc truy xuất giá trị theo thứ tự index 0..n. Ví dụ
//     println!("Giá trị đầu tiên là {}, giá trị cuối cùng là {}", arr[1], arr[3]);

//     println!("Mời bạn nhập số từ 0 tới 3");
//     let mut index = String::new();
//     io::stdin()
//     .read_line(&mut index)
//     .expect("Không đọc được giá trị vừa nhập, vui lòng nhập 1 số nguyên");

//     let index: usize = index
//     .trim()
//     .parse()
//     .expect("Dữ liệu cần nhập dạng số");
//     let element = arr[index];
//     println!("Giá trị bạn muốn tìm có phải là {}", element);

// }
// Hết bài 12..............................................................................





// Bài 10: Định nghĩa kiểu dữ liệu Enum.......................................

// (1) Enumerate sẽ làm nhiệm vụ liệt kê tất cả các biến thể. 
// VD: Giới tính: Male/Female/Neutral

// (2) Cách khai báo

// enum gioi_tinh {Male, Female, Neutral,}

// (3)Cách tạo các biến thuộc các biến thể tromg Enum

// let girl = gioi_tinh::Female;
// let boy = gioi_tinh::Male;
// let lgbt = gioi_tinh::Neutral;
// Dấu :: thể hiện Female là associated function/variant của trường trước đó.
// Khai báo ntn có ưu điểm là các kiểu dữ liệu trong Enum sẽ cùng loại

// (4) Khai báo function với Enum:
// fn music_band(band: gioi_tinh) {}

// Gọi function
// music_band(gioi_tinh::Male);
//Ví dụ ............................................

// enum Gioitinh {
//     Male,
//     Female,
//     Neutral,
// }


//     fn main() {
//         let gioi_tinh_nu: Gioitinh = Gioitinh::Female;
//         match gioi_tinh_nu {
//             Gioitinh::Female => println!(" Chúng mình thích con trai!"),
//             Gioitinh::Male => println!(" Chúng mình thích con gái!"),
//             Gioitinh::Neutral => println!(" Chúng mình thích cả 2!"),
//         }

// }
// .......................................................

//Digging deeper...........................................
// #[derive(Debug)]
enum Female {
    a(String, i32),
    
}

// #[derive(Debug)]

enum Gioitinh {
    Male(String), 
    Female(Female),               
    Neutral(i32, i32),
}
fn main() {

let nam = Gioitinh::Male("thể dục thể thao".to_string());
let huong = Gioitinh::Female(Female::a(String::from("đánh son"), 2));
let hung = Gioitinh::Neutral(2,5);

fn kiemtra(sex: Gioitinh) {
    match sex {
        Gioitinh::Male(yeuthich1) => println!("Yêu thích là {}", yeuthich1),
      //  Gioitinh::Female(girl) => println!("Sở thích là {:?} và chải tóc mỗi ngày", girl::Female::a),
      Gioitinh::Female(Female::a(sothich, b)) => println!("Sở thích là {} và chải tóc {} mỗi ngày", sothich, b), //Gioitinh::Female(Female::a(sothich, b)) Cách gọi dữ liệu Enum trong Enum
        Gioitinh::Neutral(a, b) => println!("Tớ thích tắm {} lần/ngày", a + b),
    }
}
kiemtra(nam);
kiemtra(huong);
kiemtra(hung);
}
//Digging deeper end.......................................

//(5)Enum giúp giải quyết các lỗi liên quan tới null. (Trong RUST không có null)
// Hết bài 10...........................................................................................................


//Fun test: User nhập giá trị cho Enum................................................................................
// use std::io;

// enum Gioitinh {
//     Male(String),
//     Female(String, i32),
//     Neutral(i32, i32),
// }
// fn main() {
//     println!("Tên của bạn là gì?");
//     let mut ten = String::new();
//     io::stdin()
//     .read_line(&mut ten)
//     .expect("Mời bạn nhập lại tên");

//     let ten: String = ten.trim().to_string();
//     println!("Chào {} !", ten);

//     println!("Sở thích của bạn là gì?");
//     let mut sothich2 = String::new();
//     io::stdin()
//     .read_line(&mut sothich2)
//     .expect("Mời bạn nhập lại sở thích");
//     let sothich2: String = sothich2.trim().to_string();

//     println!("Chào {}, tớ biết bạn thích {} rồi nhé", ten, sothich2);

    

// fn kiemtra(sex: Gioitinh) {
//     match sex {
//         Gioitinh::Male(yeuthich) => println!("Yêu thích là {}", yeuthich),
//         Gioitinh::Female(sothich, c) => println!("Sở thích là {} và chải tóc {} mỗi ngày", sothich, c),
//         Gioitinh::Neutral(a, b) => println!("Tớ thích tắm {} lần/ngày", a + b),
//     }
//     let ten = Gioitinh::Male(sothich2);
    
// }
// kiemtra(ten);
// kiemtra(huong);
// kiemtra(hung);
// }}
//Fun test end..................................................................................................
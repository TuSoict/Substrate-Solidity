fn main() {
    println_devide(51);

    //Bai 15 Shadowing

    /**
     * Shadowing dc dung voi tu khoa let khac mut o diểm khi không muốn thay doi gia tri cho bien, khong muon chuyen quyen so huu
     * Shadowing con co the thay doi kieu du lieu cho bien
     * 
     */

     let mut x = 5;

     {
         x = 10;
         println!("giá trị của x là : {}", x); // x return ve 10
     }

     println!("giá trị của x là : {}", x); // x return ve 10 mặc dù đã ra ngoài scope block code nhưng x đã bị thay ddoooir giá trị thành 10 neus dùng mut

     let mut y = 5;

     {
         let y = 10;
         println!("giá trị của y là : {}", y); // y return ve 10
     }

     println!("giá trị của y là : {}", y); // y return ve 5 mac du tu khoa mut khai bao luc dau nhung trong block code dang dung shadowing voi keyword let

     let test_type_varibale = "fdfsdfdsfdfs";

     println!("day la kieu chuoi: {}", test_type_varibale);

     let test_type_varibale = y;

     println!("Đay đã dc chuyển thành kiểu số với shadowing: {}", test_type_varibale);


}

fn println_devide (num: u32) {
    let mut count = 0;
    for n in 1..num {
        if check_sum(n) {
            println!("So {} chia het cho 3", n);
            count += 1;
        } else {
            println!("So {} khong chia het cho 3", n);
        }
    }

    println!("Tu 1 den {}. Co tat ca {} so chia het cho 3", num, count);
}

fn check_sum (n: u32) -> bool {
    n % 3 == 0
}
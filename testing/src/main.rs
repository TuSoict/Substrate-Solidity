mod impl_struct;

fn main() {
    // ARRAY
    {
        let array = [7; 11];
        for i in array.iter() {
            println!("{}", i);
        }
        let array_str = ["a", "b", "c", "d"];
        for i in array_str.iter() {
            println!("{}", i);
        }
        println!("first element: {}", array_str[0]);
        println!("last element: {}", array_str.last().unwrap());
    }

    // IMPL 
    {
        let quang = impl_struct::Human{
            name: String::from("Quang"),
            height: 177,
            weight: 70,
            is_male: true
        };
        quang.detail();
        quang.sleep();
        quang.hobby();
    }
    
    // STRING and &STR
    {
        let str = "Quang";
        println!("{}", str);
        let mut str2 = String::from("Quang");
        str2 = str2 + "D";
        str2.push('o');
        str2.push_str("Dang");
        println!("{}", str2);

        let str3 = "Chào/buổi/sáng";
        for i in 0..str.len() {
            println!("tại vị trí {} của chuỗi: {}", i, str.chars().nth(i).unwrap());
        }
        for sub_string in str3.split("/") {
            println!("{}", sub_string);
        }

        let mut str4 = String::with_capacity(12);
        println!("Capacity before pushing: {}", str4.capacity());
        let sub_str1 = "123456789012";
        str4.push_str(sub_str1);
        println!("Capacity after pushing string length {}: {}", sub_str1.len(), str4.capacity());
        let sub_str2 = "1";
        str4.push_str(sub_str2);
        println!("Capacity after pushing one more string lenth {}: {}", sub_str2.len(), str4.capacity());
        // capacity < length < capacity*2 --> the capacity of this string will automatically double
        println!("Length: {}", str4.len());

        let empty_str = "";
        if empty_str.is_empty() {
            println!("This string is empty");
        }
        else {
            print!("This string is not empty");
        }
    }
    
}

fn main() {
    println!("Hello, world! Đây là chương trình ví dụ về hàm và cách sử dụng hàm");
    println!("Giá trị của hàm funtion_01 là: {}", funtion_01(1, 2, 3));
}

fn funtion_01(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}
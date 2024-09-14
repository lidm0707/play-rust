

// fn take_owner(x:String){
//     print!("{x}");   
// }

fn borrow(x:&str){
    println!("{x}");   
}

fn take_owner_number(x:i32){
    println!("{x}");   
}


fn main() {
    let word = "test play".to_string();
    let slice = word;
    // take_owner(slice);
    println!("{}",&slice[0..1]);
    //take_owner(slice);
    borrow(&slice);
    println!("{}",&slice);
    //println!("{}",word);

    // &str ชี้ไปที่ to_string แต่ String คือตัวมัน ถ้า take owner ไปแล้วจะ จะเบิด


    let number = 1;
    let num2 = number ;
    take_owner_number(number);
    println!("{}",num2);
    println!("{}",number);


    /*
    ใน Rust, ประเภทตัวเลขเช่น `i32` เป็นประเภทที่ "Copy" ได้ (`Copy` trait) ซึ่งหมายความว่าเมื่อคุณกำหนดค่า `i32` จากตัวแปรหนึ่งไปยังอีกตัวแปรหนึ่ง ค่าจะถูกคัดลอกแทนที่จะมีการย้ายเจ้าของ (ownership) เหมือนประเภทที่ไม่ใช่ `Copy`

ตัวอย่างเช่น:

```rust
fn main() {
    let x: i32 = 5;
    let y = x; // คัดลอกค่า x ไปที่ y
    
    println!("x: {}, y: {}", x, y); // สามารถใช้ทั้ง x และ y ได้
}
```

ในกรณีนี้ การกำหนด `let y = x` จะไม่ย้ายเจ้าของของ `x` ไปยัง `y` แต่จะคัดลอกค่าของ `x` ทำให้ทั้ง `x` และ `y` สามารถใช้งานได้พร้อมกัน

ตรงกันข้ามกับประเภทที่ไม่สามารถ `Copy` ได้ เช่น `String`:

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // ย้าย owner ship จาก s1 ไป s2
    
    // println!("{}", s1); // ไม่สามารถใช้ s1 ได้อีกต่อไปเพราะ owner ship ถูกย้ายไปแล้ว
}
```

ในกรณีนี้ การกำหนด `s2 = s1` จะย้ายเจ้าของของ `s1` ไปยัง `s2` แทน
     */
}

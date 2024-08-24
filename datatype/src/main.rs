use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    return type_name::<T>();
}

// String slice
// char
// usize
// i32
// owner
// arr = vec

fn create_star(number: usize) -> () {
    for i in 0..number {
        let repeated_string: String = "*".repeat(i * 2 + 1);
        let spaces: String = " ".repeat(number - i - 1);
        println!("{spaces}{repeated_string}");
    }
}

fn create_arr() -> Vec<&'static str> {
    let mut vec: Vec<&str> = Vec::new();
    vec.push("a");
    return vec; // คืนค่าอาร์เรย์ arr
}

struct Person {
    name: String,
    age: i32,
}

fn main() {
    let name: &str = "RustS"; // more char
    let string1: char = 'r'; // one char
    let string2: char = 'u';
    let string3: char = 's';
    let string4: char = 't';
    let str_edit: String = "editr string rust".to_string();
    let str_edited: &str = &str_edit[3..5];
    let num: i32 = 10;
    let boo: bool = true;
    let typestring: &str = type_of(&string1);
    let person: Person = Person {
        name: "hma".to_string(),
        age: 32,
    };
    
    println!("hello {name}");
    println!("{string1} {string2} {string3} {string4}");
    println!("num is {num} boo is {boo} ");
    println!("string 1 is {typestring} can't edit it");
    println!("strEdit is {str_edit} edit to {str_edited}");
    create_star(5);
    let mut arr: Vec<&str> = create_arr();
    arr.push("b");
    println!("{arr:?}");
    println!("Person {} Age {}", person.name , person.age);
}

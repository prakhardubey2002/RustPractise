use std::collections::HashMap;//calling or using inbuilt library of hashmap in rust
fn main() {
    let mut map = HashMap::new();//new keyword is used to create instance of hashmap library
    map.insert(0, "HI1");
    map.insert(1, "HI2");
    println!("{:?}",map);

    match map.get(&0){// to match we give index instead of value in hashmap match

        Some(str1) => println!("{}",str1),
        None => println!("Doesn't Exist"),//conditionally printed statement is used here if there is no value none part will execute and if string is detected ny some then some part will execute
    }
    match map.get(&2){
        Some(str2) => println!("{}",str2),
        None => println!("Doesn't Exist"),
    }
    map.remove(&0);
    println!("{:?}",map);
}

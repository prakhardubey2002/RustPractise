fn main(){
    //unsigned integer mean integer that can't be negative and u8,u16,u32,u64 mean the amount of bit they can hold like u8 can hold 8 bit
    let unsigned:u8 = 10;
    //signed integer mean integer can be both positive or negative and i8,i16,i32,i64 mean the amount of bits that can be stored in it for example i8 can 8 bit
    let signed:i8 = -100;
    //f32 is bit storing capacity of float variable and if one want to declare whole number it should be with .0 like 1.0,2.0 
    let float:f32 = 1.2;
    println!("unsigned:{},signed:{},float:{}",unsigned,signed,float);
    //char -can only be one one word 
    let letter = "c";
    //rust support unicode so smiling emoji can be printed by following code
    let emoji = "\u{1F600}";//smiling emoji's unicode
    println!("letter:{},emoji:{}",letter,emoji);
    let is_true:bool=true;
    println!("isTrue:{}",is_true)
}
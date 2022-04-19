
fn divide(dividend:i32,divisor:i32)-> Option<i32> {
 if dividend % divisor !=0 {
     None//None to indicate failure or lack of value
 }
 else{
     Some(dividend/divisor)//Some(Value),a tupe struct that wraps a value with type t
     }
}
fn main(){
    let divide1:Option<i32>=divide(4,2);
    println!("{:?} 's unwrap form: {}",divide1,divide1.unwrap());

    // let divide2:Option<i32> = divide(2,3 );
    //unwrapping none will give "Panic output" which is exception version of rust
    // println!("{:?} 's unwrap form: {}",divide2,divide2.unwrap());//this will crash the program as none can not be unwrapped
}
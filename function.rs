fn main(){
    println!("{}",is_even(1));
}
pub fn is_even(num:u8)-> bool{//all function are private until pub is added to make it public
    let digit:u8 = num%2;
    digit==0//return boolean 
}
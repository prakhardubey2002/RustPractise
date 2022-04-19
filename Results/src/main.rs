#[derive(Debug)]
enum MyError{
    Error1
}
fn divide(dividend:i32,divisor:i32)-> Result<i32,MyError> {
    if dividend % divisor !=0 {
        Err(MyError::Error1)
    }
    else{
       Ok(dividend/divisor)
        }
   }

fn main(){
    let divide = divide(4,2);
    let res = divide.expect("we crashed");
    println!("{}",res);
}
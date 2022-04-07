fn main() {
  let a      = 5;
  let d: i32 = 25;
 
  let sum = return_sum(a,  d);
  println!("The sum of a &  b is = {}", sum);
}
 
fn return_sum(i: i32, j: i32) -> i32 {
  i + j
}
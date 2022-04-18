fn main() {
    let mut vec: Vec<i64>=vec![1,2,3,4,5];//we have refrence Vec<T> to use vector,vector is dyanamic form array which support multiple method like index,remove,push ets
    let a= vec.len();//yield length of vector
    println!("{}",a);
    let c = vec[0];//return varianle at the given index
    println!("{}",c);
    vec.push(8);
    vec.remove(0);//remove wethod work with index here 1st element is removed
    println!("{:?}",vec);
    let b= vec.len();
    println!("{}",b);
}

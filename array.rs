fn main(){
    let arr:[u8;4]=[1,2,3,4];//[u8;4] mean here are 4 unsigned integer in array with 8bit capacity each 
    let other_arr:[u8;5]=[100;5];//will print 100 5 times because length offered here is 5 
    println!("Index of arr is : {},Length of other_arr is : {} ",arr[0],other_arr.len());//len offer length of the array assigned with it
    println!("{:?},",other_arr);// to print whole array {:?} is used 
    println!("{:?},",arr);
}
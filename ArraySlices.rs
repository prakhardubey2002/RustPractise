fn main(){
    let arr = [0,1,2,3];//length is defined but not in case of slice
    let slice = &arr[1..3];//we have to derefrence the array we want to slice with &,first value of slice function is inclusive and last value is exclusive so this slice will give [1,2],but main deifference behing array and slice is that we dont know length of slice function in compile tim e so in function utilizing slice in it we dont define length 
    borrowing_slice(arr,slice); 
}
fn borrowing_slice(arr:[u8;4],slice: &[u8]){
    println!("{:?}",arr);
    println!("{:?}",slice);
    println!("length: {}",slice.len());
    println!("{} {}",slice[0],slice[1]);
}
fn main(){
    let mut i  = 0;//i is made mutable because in while loop i value is incrementing 
    while i < 4{
        println!("{}",i);
        i+=1;//i=i+1
        if i==3 {
            println!("exit");
            break;//break execution of code
        }
    }
}
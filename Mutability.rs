fn main(){
    let mut num =5;//mut let compiler know that we want to change or mutate value in our code aftewards,rust compiler follow top to bottom approach so if  value change midway of program,from that point program will be compiled with new value for example till line 8 mut n=8 at line 9 n-10 so from line 9 program will compile for value 9
    num = 3;
    println!("{}",num);
}
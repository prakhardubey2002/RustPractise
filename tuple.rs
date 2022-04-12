fn main(){
    //tuple has ability to mutiple data type is array like format 
    let tuple:(u8,bool,f32)=(5,true,2.1);
    //for tuple we use () instead of []
    let tuple2=(3,5);
    //print structure of array
    println!("First: {},second: {},Third: {} ", tuple.0,tuple.1,tuple.2 );
    //For printing complete tupe {:?} is used
    println!("{:?}",tuple2);

    //Destructuring of tuple 
    let (a,b,c)=tuple;
    println!("First: {},second: {},Third: {} ", a,b,c )
}
fn main(){
    //there are two typeof string in c primary difference between them is memory managment
    let str:&str ="hello world";//1st type of string in rust
    println!("{}",str);
    let mut string: String = String::from("Hello World");//here string rust's inbuilt object is utilized to make string,this is 2nd type of string

    let slice = &string[..6];
    println!("{}",slice.len());

    string.push('1');//push value in string ate end
    string.push_str("! Bob");// push string atend
    string.replace("Hello","bye");//replace value in string
    println!("{}",string);//print complete string string
}
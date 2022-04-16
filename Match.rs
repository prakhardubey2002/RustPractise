fn main(){// this program will run only in cargo
    let i = 4;
    match i{
        0 => println!("0"),//comparison
        1 | 2 => println!("1,2"),//or
        3..=4 => println!("3,4"),//range
        _ = >println!("default")//default value
    }
}
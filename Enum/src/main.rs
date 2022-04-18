fn main() {
    let a:MyEnum= MyEnum::A;//fields of enum are called ny refrencing using ::
    let b:MyEnum= MyEnum::B(5);
    let c:MyEnum=MyEnum::C{x:10,y:20};
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);
    //extracting enum value through comaparison using if
    if let MyEnum::B(val)= b{// ib b tye is instance of enum field B the this if will execute
        println!("{}",val);//
    }
    

    if let MyEnum::C{x,y}= c{//if c is instance of C type of enum field the this if will execute
        println!("{} {}",x,y);
    }
}

#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C {x: i32,y: i32}
}
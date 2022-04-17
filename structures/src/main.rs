fn main(){
    let name =String::from("Parrot");
    let wings =String::from("clean and long");
    let bird = Bird {
        name:name,
        attack:6,
        feather:wings
    };
    bird.print_name();
}

struct Bird{
    name:String,
    attack:u64,
    feather:String
}
impl Bird{//impl mean implementation of method here in struct we define a function
    fn print_name(&self){//here self is creating instance of its data like working with object with "This" in javascript
        println!("{}",self.attack);
        println!("{}",self.name);
        println!("{}",self.feather);
    }
}
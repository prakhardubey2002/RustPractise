fn main(){
    let name =String::from("Parrot");
    let wings =String::from("clean and long");
    let bird = Bird {
        name:name,
        attack:6,
        feather:wings
    };
    bird.print_name();
    println!("{} {}",bird.can_fly(),bird.is_animal());
}

struct Bird{
    name:String,
    attack:u64,
    feather:String
}
impl Bird{//impl mean implementation of method here in struct we define a function
    fn print_name(&self){//here self is creating instance of its data like working with object with "This" in javascript
        println!("{}",self.attack);
    }
}

impl Animal for Bird{// for is like extend
    fn can_fly(&self) ->bool {
        true
    }
}
trait Animal{//traits offer capablilty of inhertiance
    fn can_fly(&self) -> bool;
    fn is_animal(&self)->bool{
        true
    }
    
}
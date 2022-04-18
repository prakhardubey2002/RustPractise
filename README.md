[Rust](https://www.rust-lang.org/) is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

---

> Rust is idle for smartcontract in solana and Near chain and Rust's WASM(Webassembly) has capability to supercharge Javascript

---

## Installing rust

Download [Rustup-init.exe](https://www.rust-lang.org/learn/get-started) for windows

For Window Subsystem for linux

```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Hello world in Rust

```Rust
fn main() {
    println!("Hello World");
}
```

## How to compile rust program?

```bash
  rustc ./main.rs
```

```bash
  .\main.exe
```
## Compilation error due to linker
#### Linking with link.exe failed: exit code: 1

(If this error comes up during compilation )

```bash
  rustup toolchain install stable-x86_64-pc-windows-gnu
```

then

```bash
  rustup default stable-x86_64-pc-windows-gnu
```

> If error still persist then install latest VS

## What is Cargo?

Cargo is package system of rust just like npm for javascript 

Command for cargo hello world program-

```bash
  cargo new hello
```

Files created are a src folder and .toml file,src posses rust program and .toml can be taken as package.json file for simple understanding

```bash
 cd .\hello\
 cargo build
```

This will create a target folder inside hello folder created by rust's cargo which will have files like debug folder,cargo.lock,.rustc_info.json
to run created exe file by cargo build-

```bash
 cargo run
```

## How to print variable?

In println use curly braces {} in places where you want to pass string followedby string name after a comma for example-

```Rust
 fn main(){
   let a = 20;
   let b = 15;
   println("Hello ,World,{}{}",a,b);
 }
```

## Variable in Rust

###### Unsigned Integer

Unsigned integer mean integer that can't be negative and u8,u16,u32,u64 mean the amount of bit they can hold like u8 can hold 8 bit

```Rust
fn main() {
    let unsigned:u8 = 10;
    println!("Unsigned:{}",unsigned);
}
```

###### Signed integer

Signed integer mean integer can be both positive or negative and i8,i16,i32,i64 mean the amount of bits that can be stored in it for example i8 can 8 bit

```Rust
fn main() {
    let signed:i8 = -10;
    println!("signed:{}",signed);
}
```

###### Float

Declartion is same for float but instead for "i" and "u" "f" is used like F32

```Rust
fn main() {
    let flaot:F8 = 1.0;
    println!("Float:{}",Float);
}
```

###### Unicode

Rust support unicode so smiling emoji can be printed by declaring "\u{1F600}"

```Rust
fn main() {
    let emoji = "\u{1F600}";
   println!("emoji:{}",emoji);
}
```

###### Boolean

Boolean can be declared using Bool after declaration of variable with semi-colon in between

```Rust
fn main() {
     let is_true:bool=true;
    println!("isTrue:{}",is_true)
}
```
## Array in rust
Array declartion is followed by bracket having integer(i8,u8 ets) or float type(F32) and array length with colon in between

```Rust
fn main() {
   let arr:[u8;4]=[1,2,3,4];
   println!("{:?},",arr);
}
```

###### Length of array
```Rust
fn main() {
   let arr:[u8;4]=[1,2,3,4];
   println!("length is {}",arr.len());
}
```
###### Slice in array
Array can be broken down using slice by derefrencing main array.The first value passed in slice is inclusive and last value added is exclusive but primary difference behind array and slice is that slice length is not known during compile time but array's length is known so when passing a slice in a function length is not definedand "&" value is added so that compiler can unerstand it has derefrenced value
```Rust
fn main() {
   let arr = [0,1,2,3];
    let slice = &arr[1..3];
    println!("{:?}",slice);//[1,2]
}
```
## Tuple in rust
Tuple has capability to hold different data type in array like format
```Rust
fn main() {
   let tuple:(u8,bool,f32)=(5,true,2.1);
   println!("{:?}",tuple);
}
```

###### Destructuring of Tuple
```Rust
fn main() {
   let tuple:(u8,bool,f32)=(5,true,2.1);
   let (a,b,c)=tuple;
   println!("First: {},second: {},Third: {} ", a,b,c )
}
```
## Function in rust
Function in rust are by default private until "pub" is added to make them public 
In below function (num:u8) is unsigned integer of 8 bit that can this function take and boolean is data type that this function will return after executing its logic

```Rust
fn main() {
   println!("{}",is_even(1));
}
pub fn is_even(num:u8)-> bool{
    let digit:u8 = num%2;
    digit==0
}
```
## Mutability 
Rust compiler does not allow its variable to be updated or changed after its declaration but it can be changed or mutated using keyword "mut" between let and variable name

```Rust
fn main(){
    let mut num =5;//mut let compiler know that we want to change or mtate value in our code aftewards
    num = 3;
    println!("{}",num);
}
```
## Conditionals in rust
If,Else,while,for condition are same in rust as other language only difference is that condition does'nt have parenthesis 
```Rust
fn main(){
   let n:u8=4;
    if n>3 {
        println!("Greater than 2");
    }
    else n<0 {
        println!("Less than 2");
    
    }
    for i in 0..6{   
        println!("{}",i);
    }
}
```
## Match
Similar to switch statement in most languages rust have "Match" which is used for comaprison between mutiple cases with different values and condition
```Rust
fn main(){
    let i = 4;
    match i{
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ =>println!("default")
    }
}
```

## Structures 
Structure give us ability define an object with  capablilty to hold multiple data type in an definite user-defined structure,method inside structure are implemented using keyword "impl"
```Rust
  fn main(){
    let name =String::from("CorolinaParakit")
    let bird = Bird {
        name:name,
        attack:6
    };
    bird.print_name();
}

struct Bird{
    name:String,
    attack:u64,
}
impl Bird{//impl mean implementation of method here in struct we define a function
    fn print_name(&self){
        println!("{}",self.attack);
    }
}
```

###### Traits (Inhertiance of rust)
Trait let us extend structure type by adding data/function into structure
```Rust
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
impl Bird{
    fn print_name(&self){
        println!("{}",self.attack);
    }
}

impl Animal for Bird{
    fn can_fly(&self) ->bool {
        true
    }
}
trait Animal{
    fn can_fly(&self) -> bool;
    fn is_animal(&self)->bool{
        true
    }
    
}
}
```
## Vector
Vector is dynamically manipulatable form of array that come from standard library of rust which support method like push,remove,index ets
```Rust
fn main(){
    let mut vec: Vec<i64>=vec![1,2,3,4,5];
    vec.len();
    vec.push(8);
    vec.remove(0);
}
```

## HashMap
Hashmap are similar to array but here every lement has key index and element are addresed in method using refrence to there index like &2
```Rust
use std::collections::HashMap;
    fn main(){
        let mut map = HashMap::new();

         map.insert(0, "HI1");
         map.insert(1, "HI2");
         println!("{:?}",map);
         
         map.remove(&0);
         println!("{:?}",map);
}
```


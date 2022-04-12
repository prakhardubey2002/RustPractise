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

## linking with link.exe failed: exit code: 1

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

## Variable is Rust

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

Declartion is same for float but instead for iand u f is used like F32

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

boolean can be declared using Bool after declaration of variable with semi-colon in between

```Rust
fn main() {
     let is_true:bool=true;
    println!("isTrue:{}",is_true)
}
```
## Aray is rust
Array declartion is followed by bracket having integer(i8,u8 ets) or float type(F32) and array length with colon in between

```Rust
fn main() {
   let arr:[u8;4]=[1,2,3,4];
   println!("{:?},",arr);
}
```
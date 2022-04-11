[Rust](https://www.rust-lang.org/) is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.
***
>Rust is idle for smartcontract in solana and Near chain and Rust's WASM(Webassembly) has capability to supercharge Javascript 
---
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
>If error is still persist then install latest VS 

## Wahat is Cargo?
Cargo is package system of rust just like npm

command for cargo  hello world program 
```bash
  cargo new hello
```
files created are a  src folder and .toml file,src posses rust program and .toml can be taken as package.json file for simple understanding

```bash
 cd .\hello\
 cargo build
```
This will create a target folder inside hello folder created by rust's cargo which will have files like debug folder,cargo.lock,.rustc_info.json 
to run created exe file by cargo build-
```bash
 cargo run
```
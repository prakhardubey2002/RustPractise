Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.
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

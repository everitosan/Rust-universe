# Macros

Los macros son una especie de meta programación en rust que se ejecutan y expanden en tiempo de compilación.

Existen dos tipos de macros en rust, los [declarativos](./1-Declarative.md) y los [procedurales](./2-Procedural.md).

```rust
println!(); // -> declarativo 

vec![]; // -> declarativo

#[derive( ... )] // -> Procedural derive

#[custom_macro] // -> Procedural de atributo
fn public() {

}
```

## Herramientas
- [Cargo expand](https://github.com/dtolnay/cargo-expand)

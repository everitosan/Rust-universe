# Macros procedurales

Existen tres tipos de macros procedurales:
- Function
- Derive
- Attribute

Primero debe activarse el `proc-macro` en el archivo `Cargo.toml`, esto nos permitirá usar el crate `proc_macro` más adelante.

```toml
[lib]
proc-macro = true
```


### Function

El macro más simple es uno que devuelve el mismo`Token Stream` que recibe.

```rust

// lib.rs
extern crate proc_macro;

#[proc_macro]
pub fn log_time(input: TokenStream) -> TokenStream {
	input
}

// main.rs
fn main() {
	log_time!(println!("Inspector"));
}
```

Lo anterior, terminará sustituyendo el macro por el `Token Stream` en la función main después de la compilación.

```rust 
// main.rs
fn main() {
	println!("Inspector");
}
```

### Attribute
Este tipo de macros pueden recibir atributos en su compilación.

La función del macro recibirá  dos parámetros de tipo `TokenStream` y devolverá a su vez otro `TokenStream` que reemplazará la función a la que esté asociado.

```rust
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
	input
} 
```

Para poder parsear los argumentos (atributos) usados en el macro, nos apoyaremos de las bibliotecas  [darling](https://crates.io/crates/darling)  y [syn](https://crates.io/crates/syn).

```toml
// Cargo.toml

darling = "0.13"
syn = { version="2.0.75", features = ["full"]}
```


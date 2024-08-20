# Macros declarativos

También son conocidos  como `macros por ejemplo` y usan una especie de `match` para funcionar proveniente del macro `macro_rules!`.

Ejemplo de macro llamado `hello`.

```rust
#[macro_export] // Nos permite exportar el macro hello
macro_rules! hello {
	() => {
		println!("Hola mundo");
	};
}
```

- `#[macro_export]` Nos ayuda a exportar el macro de la biblioteca.
- El match se hace cuando no hay parámetros '( ) => { }'para este macro, por lo que puede usarse desde main de la siguiente manera.

```rust 
use declarative::hello;

fn main() {
	hello!();
}
```

## Metavariables

A la hora de pasar información a nuestros macros, es importante considerar las [metavariables](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables).   

Estas representan los tipos de datos que podemos mandar a las macros, algunas de ellas son:
- ty *(Type)*
- expr *(Expression)*
- block (Block expression)


El macro `map`, cuenta con dos formas en que puede ser utilizado; esto se logra gracias al match especificado en el `macro_rules!`.

```rust
use declarative::map;
use std::collections::HashMap;

...

let months = map!(i32, String);

let months2 = map!(
	1 => "Enero".to_owned(),
	2 => "Febrero".to_owned(),
	3 => "Marzo".to_owned()
);
```

Definición del macro:

```rust
#[macro_export] // Nos permite exportar el macro map
macro_rules! map {
	($key: ty, $val:ty) => { // Match para iniciar un HashMap en base a los tipos.
		// Importante notar que se devuelve un block
		{
			let map: HashMap<$key, $val> = HashMap::new();
			map
		}
	};
	($($key: expr => $val: expr),*) => { // Match para iniciar un HashMap en base a los valores.
		{
			let mut map = HashMap::new();
			$( map.insert($key, $val); )*
			map
		}
	}
}
```
# Anotaciones Rust 🦀

## Variables y Constantes 📓

Para nombrar variables en Rust, se usa la palabra reservada `let`. Por defecto las variables son inmutables y estas no se pueden cambiar después de definirse solo son "readonly".

Para escribir sobreescribir una variable se usa la palabra reservada `mut` después de `let`.

```rust
let nombre = "Jesús";
let mut nombreCompleto = "Jesús Alejandro";
```

Se pueden tener dos variables con el mismo nombre. Los tipos de datos se pueden omitir pues el lenguaje los infiere dependiendo el valor que se le asigne pero también es posible asignar un tipo de dato.

```rust
let valor = "Juan";
let valor = 12;

println!("El valor: {}", valor);
```

En este caso 🙋‍♂️ tendremos un error debido a que se toma la segunda varibale `valor` en la impresión por consola y la primera no se usa, y en Rust no se deben dejar variables y constantes sin usar.

Las constantes se declaran con la palabra reservada `const` y estas como en otros lenguajes la convesión es que el nombre sea en mayusculas (Rust te obliga a hacer lo) y deben tener un tipo de dato, no se infiere como en las variables.

```rust
const PI: f32 = 3.1416;
println!("valor de Pi: {}", PI);
```

## Tipos de datos 🧮

En Rust existen diferentes tipos de datos:

- Enteros:
  - Entero con signo 👉 `i8 al i128`
  - Entero sin signo 👉 `u8 al i128`

Los números enteros se definen con la cantidad de bits que le corresponde (depende del tamaño del número a almacenar), el limite son 128 bits.

- Flotantes:
  - Flotante con o sin signo 👉 `f32 y f64`

Los números flotantes solo soportan tamaño de 32 y 64 bits y estos pueden almacenar valores con o sin signo.

- Boolean:
  - Boolean 👉 `bool`
- Caracter:
  - Character 👉 `char`.

Los tipos caracteres en Rust se usan con la palabra reservada `char` y tienen una longitud de 4 bytes. A diferencia de otros lenguajes que solo usan 1 byte (8 bits). Esto permite almacenar cualquier valor unicode incluyendo emojis 🦀.

- Tipos compuestos:
  - Tuplas 👉 `(f32, char, &str, bool)`
  - Array 👉 `[i32, i32, i32]`

Las tuplas tienen un tamaño definido y son inmutables, en cambio pueden almacenar datos de diferentes tipos. Estas se pueden desestructurar como en `js` para acceder a sus elementos.

Los Array es un colección de elementos con el mismo tipo de datos, un tamaño definido y su longitud no se puede cambiar.

Ejemplos tipos de datos:

```rust
let edad: i32 = 27;
let numero_gigante: i128 = 1245666;
let numero_decimal: f32 = -12.2;
let es_falso: bool = false;
let rust_emoji: char = '🦀';

// Tipo compuesto (tupla)
let datos_personales: (&str, char, i32) = ("Programador", 👨‍💻, 27);
let (puesto, genero, edad) = datos_personales;
println!(
	"La persona es {} es de genero {} y tiene {} años",
	puesto, genero, edad
);

// Acceso a un elemento especifico de la tupla
println!("Puesto: {}", datos_personales.0);

// Tipo compuesto (array)
let edades: [i32; 3] = [12, 16, 21];
println!("Primer edad: {}", edades[0]);

// Array con el mismo dato repetido N veces -> [👨‍💻, 👨‍💻, 👨‍💻, 👨‍💻, 👨‍💻]
let caracteres = [👨‍💻; 5];
```

## Funciones 📚👨‍💻

Las funciones en Rust 🦀 se incian con la palabra reservada `fn` seguido del nombre de la función y abrimos, cerramos llaves. Es igual que la función `main` del proyecto que se generó con cargo.

```Rust
fn main(){
	saludo();
}

fn saludo(){
	println!("Hola, mundo");
}
```

También se pueden pasar parámetros en las funciones y definir el tipo de dato que va a retornar, en cambio si no colocamos un `return` al final de la función pero definimos el tipo de retorno de nuestra función nos retornará la última línea siempre y cuando corresponda al tipo de retorno.

```Rust
fn main(){
	println!("{}", operacion_producto(12, 25));
	println!("{}", saludo("Juan"));
}

fn operacion_producto(primero: i32, segundo: i32) -> i32 {
	return primero * segundo;
}

// Se omite el return porque la última línea es del tipo de retorno
// y se puede omitir "return"
fn saludo(nombre: &str) -> &str {
	nombre;
}
```

## Condiciones ⚖️

La condición `if` en Rust es similar a cualquier otro lenguaje:

```Rust
let edad: i32 = 24;
if edad > 18 {
	println!("Tienes acceso");
}
else {
	println!("No tienes acceso);
}
```

Al igual que en otros lenguajes existen las condiciones a una línea o en su defecto operación ternaria, en Rust se puede definir de la siguiente manera:

```Rust
let es_mayor: bool = if edad_persona > 18 { true } else { false };
println!("{}", es_mayor);
```

Entre llaves se coloca el valor a retornar pero omitiendo la palabra `return` y el valor devuelto se almacena en una variable (igual que las operaciones ternarias en otros lenguajes).

## Bucles (Ciclos) ♻️

Existe un bucle "especial" en Rust que se llama `loop`, este crea un bucle infinito y solo se puede detener mediante un `break`. Normalmente se maneja mediante una condición y un contador (así como un while o do while en varios lenguajes) y se ve de la siguiente manera:

```Rust
let mut contador: i32 = 1;
loop {
	if contador > 10 {
		break;
	}
	println!("El contador es: {}", contador);
	contador = contador + 1;
}
```

El bucle se ejecuta hasta que el contador llegue a 10 y después de eso se rompe y termina su ejecución, mientras el contador no llegue a su limite imprimirá el valor actual del contador.

Existe el ciclo while que funciona como en otros lenguajes:

```Rust
let mut segundo_contador: i32 = 0;
let elementos: [i32; 5] = [3, 45, 23, 12, 9];
while segundo_contador < 5 {
	println!("El elemento: {}",  elementos[segundo_contador as usize]);
	segundo_contador += 1;
}
```

NOTA: el tipo `usize` indica que...

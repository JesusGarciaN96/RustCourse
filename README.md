# Anotaciones Rust 🦀
## Variables y Constantes 📓
Para nombrar variables en Rust, se usa la palabra reservada `let`. Por defecto las variables son inmutables y estas no se pueden cambiar después de definirse solo son "readonly".

Para escribir sobreescribir una variable se usa la palabra reservada `mut` después de `let`.

```
let nombre = "Jesús";	// Variable tipica en Rust
let mut nombreCompleto = "Jesús Alejandro"; // Permite sobreescribir la variable 
```
Se pueden tener dos variables con el mismo nombre. Los tipos de datos se pueden omitir pues el lenguaje los infiere dependiendo el valor que se le asigne pero también es posible asignar un tipo de dato.

```
let valor = "Juan";
let valor = 12;

println!("El valor: {}", valor);
```

En este caso 🙋‍♂️ tendremos un error debido a que se toma la segunda varibale `valor` en la impresión por consola y la primera no se usa, y en Rust no se deben dejar variables y constantes sin usar.

Las constantes se declaran con la palabra reservada `const` y estas como en otros lenguajes la convesión es que el nombre sea en mayusculas (Rust te obliga a hacer lo) y deben tener un tipo de dato, no se infiere como en las variables.

```
const PI: f32 = 3.1416;
println!("valor de Pi: {}", PI);
```

fn main() {
    tipo_datos();
}

fn tipo_datos() {
    let edad: i32 = 27;
    print!("La edad: {}", edad);
}

#[allow(dead_code)]
fn manejo_variables() {
    let mut nombre = "Jesus";
    println!("Hello, {}!", nombre);

    nombre = "Jesus Alejandro";
    println!("{}", nombre);

    let nombre = 12;
    print!("Hola, {}", nombre);

    const PI: f32 = 3.1416;
    println!("El valor de la constante PI: {}", PI);
}

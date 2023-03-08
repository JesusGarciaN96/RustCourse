fn main() {
    tipo_datos();
}

fn tipo_datos() {
    let edad: i32 = 27;
    println!("La edad: {}", edad);

    let numero_gigante: i128 = 1245666;
    println!("Super número: {}", numero_gigante);

    let resultado_operacion: f32;
    let primer_operando: f32 = -12.2;
    let segundo_operando: f32 = 25.4;
    resultado_operacion = primer_operando + segundo_operando;
    println!("El resultado es: {}", resultado_operacion);

    let rust_emoji: char = '🦀';
    println!("Mascota Rust: {}", rust_emoji);
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

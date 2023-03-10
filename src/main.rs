fn main() {
	bucles();
}

// Bucles en Rust
fn bucles(){
	let mut contador: i32 = 1;
	loop {
		if contador > 10 {
			break;
		}
		println!("El contador es: {}", contador);
		contador = contador + 1;
	}

	let mut segundo_contador: i32 = 0;
	let elementos: [i32; 5] = [3, 45, 23, 12, 9];
	while segundo_contador < 5 {
		println!("El elemento: {}",  elementos[segundo_contador as usize]);
		segundo_contador += 1;
	}

    // El método iter() permite crear el array en "iterador" para poder
    // usarlo en un bucle for in
    let cadenas: [&str; 3] = ["Juan", "Diana", "Pedro"];
    for nombre in cadenas.iter(){
        println!("El nombre es: {}", nombre)
    }

    // Iteración por rangos
    // La iteación por rango es similar a Ruby 1..10 (va del 0 al 9)
    // Si que queremos incluir el valor "10" usamos 1..=10
    // Para hacerlo de manera invertida se encapsula el rango (1..=10).rev()
    for valor in (1..=10).rev() {
        println!("El valor: {}", valor);
    }
}

// Condiciones IF en Rust
#[allow(dead_code)]
fn condiciones(edad_persona: i32){
	if edad_persona > 18 {
		println!("Eres mayor de edad");
	}
	else {
		println!("No tienes acceso al sistema");
	}

	// Condición en una linea
	let es_mayor: bool = if edad_persona > 18 { true } else { false };
	println!("{}", es_mayor);
}

// Funciones en Rust
#[allow(dead_code)]
fn suma(primer_valor: i32, segundo_valor: i32) -> i32 {
	return primer_valor + segundo_valor;
}

#[allow(dead_code)]
fn saludo_persona(nombre: &str) -> &str {
	nombre
}

#[allow(dead_code)]
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

    // Las tuplas se pueden desestructurar (como en JS)
    let datos_personales: (&str, char, i32) = ("Programador", 'H', 27);
    let (puesto, genero, edad) = datos_personales;
    println!(
        "La persona es {} es de genero {} y tiene {} años",
        puesto, genero, edad
    );
    println!("Puesto: {}", datos_personales.0);

    let edades: [i32; 3] = [12, 16, 21];
    let emojis: [char; 4] = ['🍌'; 4];

    println!("Primer edad: {}", edades[0]);
    println!("tercer banana {}", emojis[2]);
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

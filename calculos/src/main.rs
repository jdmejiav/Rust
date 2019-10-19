
use std::io;

fn main (){
	println!("Calculadora de algunas cositas jeje");
	println!("");
	loop{
		println!("1. Triángulo (1)\n2. reactángulo (2)\n3. Círculo (3)\n4. imc(4)\n5. Salir(5)");
		println!("");
		let mut entrada = String::new();

		io::stdin().read_line(&mut entrada).unwrap();
		let entrada:i8 = entrada.trim().parse().ok().expect("Hubo un error");
		if entrada!=5{
				let retorno= opcion (entrada);
				println!("{}",retorno);
		}else {
				if entrada==5{
					break;
				}
			}

		println!("");
		}

	}


fn imc (talla:f32,peso:f32) ->f32{
	return peso / (talla*talla);
}
fn opcion (_opcion:i8)->f32{

	let mut base = String::new();
	let mut altura = String::new();
	let mut radio = String ::new();
	let mut peso = String :: new ();
	match _opcion{
	//Triángulo
		1 => {
			println!("Ingrese altura del triángulo");
			io::stdin().read_line(&mut altura).unwrap();
			let altura:f32 = altura.trim().parse().ok().expect("Hubo un fallo");
			println!("Ingrese la base del triángulo: ");
			io::stdin().read_line(&mut base).unwrap();
			let base:f32 = base.trim().parse().ok().expect("Hubo un fallo");
			print!("El área del triángulo es de: ");
			return triangulo(altura,base);
		},
	//Rectángulo
		2 =>{
			println!("Ingrese altura del rectángulo");
			io::stdin().read_line(&mut altura).unwrap();
			let altura:f32 = altura.trim().parse().ok().expect("Hubo un fallo");
			println!("Ingrese la base del rectángulo");
			io::stdin().read_line(&mut base).unwrap();
			let base:f32 = base.trim().parse().ok().expect("Hubo un fallo");
			print!("El área del rectángulo es de: ");
			return rectangulo(altura,base);
		},
	//Cículo
		3=>{
			println!("Ingrese el radio del circulo");
			io::stdin().read_line(&mut radio).unwrap();
			let radio:f32 = radio.trim().parse().ok().expect("Hubo un fallo");
			print!("El área del circulo es de: ");
			return circulo(radio);
		},
	//Imc
		4=>{
			println!("Ingrese altura de la persona");
			io::stdin().read_line(&mut altura).unwrap();
			let altura:f32 = altura.trim().parse().ok().expect("Hubo un fallo");
			println!("Ingrese la Peso de la persona");
			io::stdin().read_line(&mut peso).unwrap();
			let peso:f32 = peso.trim().parse().ok().expect("Hubo un fallo");
			print!("El índice de masa corporal de la persona, es de: ");
			return imc(altura,peso);
		},
	//Default
		_opcion =>{
			println!("No es una opción válida");
			return -1.0;
		},
	}
}
fn triangulo (altura:f32,base:f32) ->f32{
	return base*(altura/2.0);
}

fn rectangulo (altura:f32,base:f32) ->f32{
	return base*altura;
}
fn circulo (radio:f32) ->f32{
	return radio*radio*3.1416;
}

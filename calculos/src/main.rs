
use std::io;

fn main (){
	
	loop{
		let mut entrada = String::new();
		io::stdin().read_line(&mut entrada).unwrap();		
		let entrada = entrada.trim();
			
		if entrada=="a"||entrada=="A"{
			println!("Calculadora de índice de masa corporal");
			println!("Ingrese talla:");
			let mut talla =String::new();
			io::stdin().read_line(&mut talla).unwrap();
			let talla: f32 = talla.trim().parse().ok().expect("Hubo un error");
			println!("Ingrese Peso: ");
			let mut peso = String::new ();
			io::stdin().read_line(&mut peso).unwrap();
			let peso : f32 =peso.trim().parse().ok().expect("Hubo un error");
			let imc = imc (talla,peso);
			println!("Su índice de masa corporal, es de: {}",imc); 
		}else{
			if entrada=="b"||entrada=="B"{
				println!("Es b");
			} else {
				if entrada=="Q"||entrada=="q"{
					break;
				}
			}	
		}
	}

}
fn imc (talla:f32,peso:f32) ->f32{
	return peso / (talla*talla);
}

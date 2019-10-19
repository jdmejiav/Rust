
use std::io;

fn main () {
	
	let mut fib :String = String::new();
	println!("Ingrese un de fibonacci a calcular: ");
	io::stdin().read_line(&mut fib).unwrap();
	let _fib: u64 = fib.trim().parse().ok().expect("Hubo un error");
	let calc = fibonacci(_fib);
	println!("El fibonacci de {}, es : {}",_fib,calc); 
}



fn fibonacci (n: u64) -> u64{
	match n {
		0=>0,
		1=>1,
		2=>1,
		_=>fibonacci(n-1)+fibonacci(n-2),
	}


}

use std::io;

fn main () {
	let mut fib :String = String::new();
	io::stdin().read_line(&mut fib).unwrap();
	fib: u32 = fib.trim().parse().Ok().Expect("Hubo un error");
 
}

  //Find the sum of all multiples of 3 or 5 below 1000.
fn main() {

  result();
  let x = result();  
  println!("{}", x );

  println!("´Dos horas intentando actualizar el repositorio de con git bash! Me falta leer!");

}


fn result() ->i32 {
	let mut sum = 0;
	for i in 1..1000 {
		if i % 3 == 0 || i % 5 == 0 {
			sum +=i;

		}	
	}
sum
}


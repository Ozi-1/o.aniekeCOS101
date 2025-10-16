fn main() {
	let p:f64 = 520000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;
	let t:f64 = r/100.00;

	//compound interest
	let a = p * ( 1.0 + t) * n;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {}", ci);

}
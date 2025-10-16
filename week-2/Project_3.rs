fn main() {
	let p:f64 = 510000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	//depreciation formula 
	let a = p * (1.0 - r/100.00).powf(t);

	println!("The value of the TV after {} years is #{:.2}", t, a);
}
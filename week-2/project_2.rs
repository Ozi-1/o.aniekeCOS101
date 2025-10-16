fn main() {
	let sales_records = [
	    ("Toshiba",2, 450000.00), 
	    ("Mac",1, 1500000.00),
	    ("HP",3, 750000.00),
	    ("Dell",3, 2850000.00),
	    ("Acer",1, 250000.00),
	];

	let total_sum: f64 = sales_records.iter()
	    .map(|(_, qty, price)| (*qty as f64) * *price)
	    .sum();

    // calculate average 
	let avg = total_sum / (sales_records.len() as f64);
	println!("Total: {:.2}, Average: {:.2}", total_sum, avg);
}

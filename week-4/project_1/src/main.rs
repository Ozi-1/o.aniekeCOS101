use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut a).expect("Failed to read input for a");
    println!("Enter the value of b:");
    io::stdin().read_line(&mut b).expect("Failed to read input for b");
    println!("Enter the value of c:");
    io::stdin().read_line(&mut c).expect("Failed to read input for c");

    let a: f64 = a.trim().parse().expect("Please enter a valid number for a");
    let b: f64 = b.trim().parse().expect("Please enter a valid number for b");
    let c: f64 = c.trim().parse().expect("Please enter a valid number for c");

    let discrimant = b * b - 4.0 * a * c;

    if discrimant > 0.0 {
        let root1 = (-b + discrimant.sqrt()) / (2.0 * a);
        let root2 = (-b - discrimant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discrimant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root:");
        println!("x = {}", root);
    } else {
        println!("No real roots, discriminant is negative.");
    }
}

use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? y/n: ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u8 = age.trim().parse().expect("Please enter a valid number");



    if experience == "y" && age >= 40 {
        let incentive:i32 = 1_560_000;
        println!("The employee's annual incentive is #{}", incentive);
    } else if experience == "y" && age >= 30 && age < 40 {
        let incentive:i32 = 1_300_000;
        println!("The employee's annual incentive is #{}", incentive);
    } else {
        let incentive:i32 = 100_000;
        println!("The employee's annual incentive is #{}", incentive);
    }

    
}

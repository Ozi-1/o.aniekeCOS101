use std::io;

fn area_trapezium() {
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("Enter height");
    io::stdin().read_line(&mut height).expect("Invalid input");

    println!("Enter base 1:");
    io::stdin().read_line(&mut base1).expect("Invalid input");

    println!("Enter base 2:");
    io::stdin().read_line(&mut base2).expect("Invalid input");

    let height: f64 = height.trim().parse().expect("Invalid input");
    let base1: f64 = base1.trim().parse().expect("Invalid input");
    let base2: f64 = base2.trim().parse().expect("Invalid input");

    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {}",area);

}

fn area_rhombus() {
    let mut d1 = String::new();
    let mut d2 = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut d1).expect("Invalid input");

    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut d2).expect("Invalid input");

    let d1: f64 = d1.trim().parse().expect("Invalid");
    let d2: f64 = d2.trim().parse().expect("Invalid");

    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

fn area_parallelogram() {
    let mut base = String::new();
    let mut height = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut base).expect("Invalid input");

    println!("Enter altitude:");
    io::stdin().read_line(&mut height).expect("Inavalid input");

    let base: f64 = base.trim().parse().expect("Invalid input");
    let height: f64 = height.trim().parse().expect("Invalid input");

    let area = base * height;
    println!("Area of Parallelogram = {}",area);
}

fn area_cube() {
    let mut side = String::new();

    println!("Enter length of the side:");
    io::stdin().read_line(&mut side).expect("Invalid input");

    let side: f64 = side.trim().parse().expect("Invalid input");
    let area = 6.0 * side.powi(2);

    println!("Area of Cube = {}", area);

}

fn volume_cylinder() {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut radius).expect("Invalid input");

    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Invalid input");
    

    let radius: f64 = radius.trim().parse().expect("Invalid input");
    let height: f64 = height.trim().parse().expect("Invalid input");

    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("Volume of Cylinder = {}", volume);

}

fn main() {
    println!("Choose a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Area of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    let choice: i32 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("Inavalid choice"),
    }
}
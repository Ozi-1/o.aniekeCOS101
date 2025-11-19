fn main() {
    // Using Vec::new()
    let v : Vec<i64> = Vec::new("1");

    //printing the size of vector 
    println!("\nThe length of Vec::new is: {}",v.len());

    //Using macro
    let v = vec!["Grace","Ozioma","Omuwa","Kelechi","Susan","Chinyere"];

    //printing the size of vector 
    println!("\nThe length of the vec macro is: {}",v.len());
}

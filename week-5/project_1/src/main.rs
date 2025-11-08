use std::io;
fn main() {
    println!("CODE       MENU                               PRICE");
    println!("P          Poundo Yam/Edinkaiko Soup          3,200");
    println!("F          Fried Rice & Chicken               3,000");
    println!("A          Amala & Ewedu Soup                 2,500");
    println!("E          Eba & Egusi Soup                   2,000");
    println!("W          White Rice & Stew                  2,500");

    loop{ 
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter the menu code (P/F/A/E/W):");
        io::stdin().read_line(&mut input1).expect("Not a valid input");
        let menu_code = input1.trim().to_uppercase();

        println!("Enter the quantity you want:");
        io::stdin().read_line(&mut input2).expect("Not a valid input");
        let quantity:f64 = input2.trim().parse().expect("Not a valid input");

        let mut price:f64 = 0.0;

        if menu_code == "P"{
         price = 3_200.00;
        } else if menu_code == "F"{
         price = 3_000.00;
        }
        else if menu_code == "A"{
         price = 2_500.00;
        }
        else if menu_code == "E"{
         price = 2_000.00;
        }
        else if menu_code == "W"{
         price = 2_500.00;
        }
        else{
            println!("Invalid menu code entered.");
            return;
        }
    
        let total_cost:f64 = price * quantity;

        if total_cost >= 10_000.00{
            let total_cost = total_cost - (total_cost * 0.05);
        
        println!("The total cost of the item is {}",total_cost);
        }

        let mut choice = String::new();
        println!("Would you like to continue? (y/n)");
        io::stdin().read_line(&mut choice).expect("Not a valid Answer");
        let choice = choice.trim().to_lowercase();

        if choice == "n"{
            break;
        }
        else if choice == "y"{
            continue;
        }

        else {println! ("INVALID INPUT");
        }

 }


}

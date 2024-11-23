use std::io;

fn main() {
    // Display the menu
    println!("Welcome to our restaurant!");
    println!("Please select from the following menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken      - N3,000");
    println!("A = Amala & Ewedu Soup        - N2,500");
    println!("E = Eba & Egusi Soup          - N2,000");
    println!("W = White Rice & Stew         - N2,500");

    // Define prices for each item
    let prices = [
        ("P", 3200),
        ("F", 3000),
        ("A", 2500),
        ("E", 2000),
        ("W", 2500),
    ];

    // Variable to store total cost
    let mut total_cost = 0;

    loop {
        // Input food type
        println!("Enter the item code (P, F, A, E, W) to order or Q to finish ordering:");
        let mut item_choice = String::new();
        io::stdin().read_line(&mut item_choice).expect("Failed to read line");
        let item_choice = item_choice.trim().to_uppercase();

        // Break loop if 'Q' is entered
        if item_choice == "Q" {
            break;
        }

        // Check if item choice is valid
        let item_price = match prices.iter().find(|&&(code, _)| code == item_choice) {
            Some(&(_, price)) => price,
            None => {
                println!("Invalid item code. Please select a valid item.");
                continue;
            }
        };

        // Input quantity
        println!("Enter the quantity:");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity, please enter a valid number.");
                continue;
            }
        };

        // Calculate and add cost for the selected item and quantity
        total_cost += item_price * quantity;
    }

    // Apply discount if total cost is greater than N10,000
    if total_cost > 10_000 {
        total_cost = (total_cost as f32 * 0.95) as u32; // 5% discount
        println!("A 5% discount has been applied for orders over N10,000.");
    }

    // Display the total charges
    println!("The total charges for your order are: N{}", total_cost);
}
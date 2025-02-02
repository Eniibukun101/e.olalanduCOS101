use std::io;

fn carbon_calculator(a: f32, b:f32) -> f32 {
    let mut input = String::new();
    println!("How long would you say, on average, you use your devices for (in hours)?");
    io::stdin().read_line(&mut input).expect("Invalid input, expected integer");
    let time: f32 = input.trim().parse().expect("Invalid input, expected decimal");

    println!("You use your devices for {:.2} hours.",time);
    let carbon_formula: f32 = time * a * b;
    println!("Your carbon footprint is {:.2} kg CO₂",carbon_formula);

    carbon_formula
}

fn main() {
    let appliance = [("P","phone",11.1,0.0675),
    ("L","laptop",70.0,0.06),
    ("T","tablet",27.5,0.04),
    ("S","smartwatch",1.65,0.0075),
    ("E","earphones",2.13,0.0035),
    ("M","modem",0.14,0.25)];

    println!("\n===CARBON FOOT-PRINT CALCULATOR===\n");
    println!("\nWelcome user to our (GROUP 10) carbon footprint calculator.");
    println!("This will be possible by calculating the greenhouse gas emission of your electronics. \n");

 let mut total_carbon_footprint: f32 = 0.0;
 loop {
    let mut input = String::new();
    println!("What devices do you use? P = Phone, L = Laptop, T = Tablet, S = Smartwatch, E = Earphones, M = Modem\n");
    io::stdin().read_line(&mut input).expect("Invalid input, please look at options");
    let device = input.trim().to_uppercase();

        if let Some(&(code, name, charge, emission_factor)) = appliance.iter().find(|&&(code, _, _, _)| code == device)
    {
        println!("\nYou selected: {}", name);
        total_carbon_footprint += carbon_calculator(charge, emission_factor);   
    }
        else {
            println!("\nInvalid selection. Please choose a valid device code (P, L, T, S, E or M).");
    }

    let mut input2 = String::new();
    println!("Do you want to continue? Yes (Y) or No (N)");
    io::stdin().read_line(&mut input2).expect("Invalid input, expected Y or N.");
    let choice = input2.trim().to_uppercase();

    if choice == "N"{
        break;
    }

    else if choice != "Y" {
        println!("Invalid choice, please select Y if you want to continue.");
    }
}

    println!("\n=== SUMMARY ===");
        println!(
        "\nYour total carbon footprint for the selected devices is {:.2} kg CO₂.",
        total_carbon_footprint);

    }
    
 
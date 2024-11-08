use std::io;

fn main() {
    // Input experience and age
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? (yes or no):");
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience = experience.trim().to_lowercase();

    println!("Enter the employee's age:");
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please enter a valid age");

    // Determine incentive based on experience and age
    let incentive = if experience == "yes" {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000 * 12 // Monthly amount converted to annual
        } else {
            0 // No criteria specified for ages between 28 and 30 for experienced
        }
    } else {
        100_000
    };

    // Output the incentive
    println!("The annual incentive is: N{}", incentive);
}

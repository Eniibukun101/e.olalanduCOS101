use std::io;
use std::f64::consts::PI;

fn main() {
    loop {
        println!("\nPick an option to perform a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice = get_input("Enter your choice (1-6):").trim().parse::<u32>();
        
        match choice {
            Ok(1) => {
                let height = get_input("Enter height:").trim().parse::<f64>().unwrap();
                let base1 = get_input("Enter base1:").trim().parse::<f64>().unwrap();
                let base2 = get_input("Enter base2:").trim().parse::<f64>().unwrap();
                println!("Area of Trapezium: {:.2}", area_of_trapezium(height, base1, base2));
            },
            Ok(2) => {
                let diagonal1 = get_input("Enter diagonal1:").trim().parse::<f64>().unwrap();
                let diagonal2 = get_input("Enter diagonal2:").trim().parse::<f64>().unwrap();
                println!("Area of Rhombus: {:.2}", area_of_rhombus(diagonal1, diagonal2));
            },
            Ok(3) => {
                let base = get_input("Enter base:").trim().parse::<f64>().unwrap();
                let altitude = get_input("Enter altitude:").trim().parse::<f64>().unwrap();
                println!("Area of Parallelogram: {:.2}", area_of_parallelogram(base, altitude));
            },
            Ok(4) => {
                let side = get_input("Enter side length:").trim().parse::<f64>().unwrap();
                println!("Area of Cube: {:.2}", area_of_cube(side));
            },
            Ok(5) => {
                let radius = get_input("Enter radius:").trim().parse::<f64>().unwrap();
                let height = get_input("Enter height:").trim().parse::<f64>().unwrap();
                println!("Volume of Cylinder: {:.2}", volume_of_cylinder(radius, height));
            },
            Ok(6) => {
                println!("Exiting the program");
                break;
            },
            _ => println!("Invalid choice! Please enter a number fromn 1 to 6."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input
}

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}

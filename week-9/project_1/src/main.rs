use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the drink categories and their items
    let lager = [
        "33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star",
    ];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Prepare the content to write to the file
    let mut content = String::new();

    content.push_str("Lager:\n");
    for drink in &lager {
        content.push_str(&format!("- {}\n", drink));
    }
    content.push('\n');

    content.push_str("Stout:\n");
    for drink in &stout {
        content.push_str(&format!("- {}\n", drink));
    }
    content.push('\n');

    content.push_str("Non-Alcoholic:\n");
    for drink in &non_alcoholic {
        content.push_str(&format!("- {}\n", drink));
    }

    // Write the content to a .txt file
    let mut file = File::create("Nigerian Brewery Limited.txt")?;
    file.write_all(content.as_bytes())?;

    println!("The file has been created!");
    Ok(())
}

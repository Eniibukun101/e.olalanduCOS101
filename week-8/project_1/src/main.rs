use std::io;

fn main() {
    let profession: usize = read_line("What's your profession (Please pick a number from 1 to 4)?
1. Office administrator
2. Academic
3. Lawyer
4. Teacher").parse().expect("Failed to parse profession");
    if profession < 1 || profession > 4 {panic!("Invalid profession entered");}
    let professions = ["Office administrator", "Academic", "Lawyer", "Teacher"];
    let profession = professions[profession - 1];
    let years_of_experience: usize = read_line("Enter your number of  years of experience:").parse().expect("Failed to parse years of experience");

    let public_servant_levels = [
        "APS 1-2",
        "APS 1-2",
        "APS 3-5",
        "APS 3-5",
        "APS 5-8",
        "APS 5-8",
        "APS 5-8",
        "EL1 8-10",
        "EL1 8-10",
        "EL2 10-13",
        "EL2 10-13",
        "EL2 10-13",
        "SES",
    ];

    let years_of_experience = if years_of_experience - 1 < 12 { years_of_experience - 1 } else { 12 };
    let public_servant_level = public_servant_levels[years_of_experience];

    println!("You are a/an {}", profession);
    println!(" Your Public Servant Level is {}", public_servant_level);
}

fn read_line(hint: &str) -> String {
    println!("{}", hint);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}
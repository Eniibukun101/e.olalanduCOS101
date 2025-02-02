use std::io;
use std::io::Read;

fn main() {
    let mut input1 = String::new();
    println!("
        Hi! Welcome to Globacom Ltd's management access application!
        Select your position (1-5) from the options below to access the data you need!
        1. Administrator.
        2. Project Manager.
        3. Employee.
        4. Customer.
        5. Vendor.");
    io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let position:u32= input1.trim().parse().expect("Failed to read input.");

        if position == 1{
            let mut file = std::fs::File::open("globacom_dbase1.sql");
            let mut contents = String::new();
            file.expect("Sorry, can't find the database you're looking for!").read_to_string(&mut contents).unwrap();
            print!("{}",contents ); 
        }

        else if position ==2{
            let mut file = std::fs::File::open("project_table.sql");
            let mut contents = String::new();
            file.expect("Sorry, can't find the table you're looking for!").read_to_string(&mut contents).unwrap();
            print!("{}",contents ); 
        }
        
        else if position ==3{
            let mut file = std::fs::File::open("staff_table.sql");
            let mut contents = String::new();
            file.expect("Sorry, can't find the table you're looking for!").read_to_string(&mut contents).unwrap();
            print!("{}",contents ); 
        }

        else if position ==4{
            let mut file = std::fs::File::open("customer_table.sql");
            let mut contents = String::new();
            file.expect("Sorry, can't find the table you're looking for!").read_to_string(&mut contents).unwrap();
            print!("{}",contents ); 
        }

        else if position ==5{
            let mut file = std::fs::File::open("dataplan_table.sql");
            let mut contents = String::new();
            file.expect("Sorry, can't find the table you're looking for!").read_to_string(&mut contents).unwrap();
            print!("{}",contents ); 
        }
       
       }

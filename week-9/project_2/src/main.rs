use std::fs::File;
use std::io::{BufWriter, Write};

// Define a struct to represent a student
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: String,
}

fn main() {
    // Create a vector of students
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: "300".to_string(),
        },
        Student {
            name: "Adams Alitu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: "100".to_string(),
        },

         Student {
            name: "Shania Bolanla".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer Science".to_string(),
            level: "300".to_string(),
        },

         Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical Engineering".to_string(),
            level: "400".to_string(),
        },

         Student {
            name: "Bianca Edemoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical Engineering".to_string(),
            level: "100".to_string(),
        },


    
    ];

    // Display student details
    println!("Student Details:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: ", i + 1);
        println!("Name: {}", student.name);
        println!("Matric Number: {}", student.matric_number);
        println!("Department: {}", student.department);
        println!("Level: {}", student.level);
        println!();
    }

    // Save student details to a file
    let file_name = "student_details.txt";
    let file = match File::create(file_name) {
        Err(why) => panic!("Couldn't create {}: {}", file_name, why),
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(file);
    for student in &students {
        let line = format!(
            "{} {} {} {}\n",
            student.name, student.matric_number, student.department, student.level
        );
        match writer.write(line.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", file_name, why),
            Ok(_) => println!("Saved {} to {}", student.name, file_name),
        }
    }
}


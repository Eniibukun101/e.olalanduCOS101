//random number generator

use rand::Rng;
use std::io;


fn main() {
    println!("Hey! What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Okay {}, pick a number from 1 to 100!",name.trim() );
    
    let correct = rand::thread_rng().gen_range(1..=100); 
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read input");

    let guess:u64 = num.trim().parse().expect("Failed to read input"); //a new variable so we can pass the user's input as an integer using parse

    if guess == correct {
        println!("You guessed the correct number!😊");
    }

        else if guess < correct {
            println!("Your guess was too small 😒");
        }

        else {
            println!("Your guess was too big 🤦‍♂️");
        }

    println!("The number was {}!",correct);
}




// //random number generator

// use rand::Rng;
// use std::{io, cmp::Ordering}; 


// fn main() {
//     println!("Hey! What's your name?");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Failed to read input");
//     println!("Okay {}, pick a number from 1 to 100!",name.trim() );
    
//     let correct = rand::thread_rng().gen_range(1..=100); 
//         let mut num = String::new();
//         io::stdin().read_line(&mut num).expect("Failed to read input");

//     let guess:u64 = num.trim().parse().expect("Failed to read input");

   
// //alternative to "if" method used in third practice

//     let output= match guess.cmp(&correct) {
//         Ordering::Equal=> "You guessed the correct number!😊",
//         Ordering::Less=> "Your guess was too small 😒",
//         Ordering::Greater=>"Your guess was too big 🤦‍♂️",
//         };

//         println!("{}",output.trim() );
//     println!("The number was {}!",correct);
// }

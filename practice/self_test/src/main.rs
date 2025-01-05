//guess the number


use std::io;
use rand::Rng;

fn main() {


        
    println!("Hey! What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let correct =rand::thread_rng().gen_range(1..=10); //the 'correct' variable is the number that will be randomly generated
    println!("Okay {}, pick a number from 1 to 10!",name.trim() );
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");

    let guess:u64 = num.trim().parse().expect("Failed to read input"); 
    // the parse changes the string input to integer and we need to create a new 
    // variable "guess" to 'parse' the input and we also had to make it an unsigned
    // value 'u64' 

    if guess==correct {
        print!("You're correct!😁");
       
    }
        else if  guess!=correct{
            

                let mut count = 0;

                    loop {
                        
                        println!("WRONG! Guess again");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input");
                        let chance:u64 = input.trim().parse().expect("Failed to read input");

                        count +=1;

                             if count ==2{
                                break;
                            }

                            if chance != correct{
                                println!("NOPE!");
                                continue;
                            }

                    }
        }

        


    
    
}




// //guess the number


// use std::io;
// use rand::Rng;

// fn main() {

//       let mut count = 0;

//     loop {
//         count +=1;

//         if count ==3{
//             break
//         }
        
//     println!("Hey! What's your name?");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Failed to read input");

//     let correct =rand::thread_rng().gen_range(1..=10); //the 'correct' variable is the number that will be randomly generated
//     println!("Okay {}, pick a number from 1 to 10!",name.trim() );
//     let mut num = String::new();
//     io::stdin().read_line(&mut num).expect("Failed to read input");

//     let guess:u64 = num.trim().parse().expect("Failed to read input"); 
//     // the parse changes the string input to integer and we need to create a new 
//     // variable "guess" to 'parse' the input and we also had to make it an unsigned
//     // valie 'u64' 

//     if guess==correct {
//         print!("You're correct!😁");
//         break;
//     }
//         else if  guess!=correct {
//             println!("LOSER!!!🤪");
//             println!("The number was {}",correct );
//             continue;
//         }
        
//     }
// }


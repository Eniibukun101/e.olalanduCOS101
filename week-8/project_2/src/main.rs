fn main() {
    // Vector of tuples to store candidate data 
    let candidates = vec![
        ("TJ", 8),
        ("Rebecca", 15),
        ("Jeremy", 12),
        ("Benji", 2),
    ];

    let mut most_experienced = candidates[0];
    for candidate in &candidates {
        if candidate.1 > most_experienced.1 {
            most_experienced = *candidate;
        }
    }

    //final output
    println!(
        "The most experienced candidate is {} with {} years of experience in programming!",
        most_experienced.0, most_experienced.1
    );
}


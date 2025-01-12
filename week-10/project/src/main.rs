struct Brand{
    name:String,
    price:u64,
}

fn main() {
    let brand1 = Brand{
        name:String::from("HP"),
        price:650000,
    };

    let brand2 = Brand{
        name:String::from("IBM"),
        price:755000,
    };

    let brand3 = Brand {
        name:String::from("Toshiba"),
        price:550000,
    };

    let brand4 = Brand{
        name:String::from("Dell"),
        price:850000,
    };


    //calculate the price of 3 of each brand
    let h:u64 =brand1.price * 3;
    let i:u64 =brand2.price * 3;
    let t:u64 =brand3.price * 3;
    let d:u64 =brand4.price * 3;
    let total = h + i + t + d;

    println!("The customer's total cost is {}",total);
}

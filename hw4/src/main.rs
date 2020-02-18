// use std::io;

fn main() {
    /*
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading input");
    println!("You selected: {}", x);

    let y = x.as_str();
    let value = value_in_cents(Coin::y);
    println!("The value of a {} is {}.", y, value);
    */
    //***Tried to get user input for this, but couldn't get it to accept the input...***

    //Get value and print:
    let value = value_in_cents(Coin::Dime);
    println!("The value is {} cents.", value);
}

#[allow(dead_code)] // Hide warnings when coins are not called. Cleans up compiler.

enum Coin { // Set up database for possible coins
    Penny,
    Nickel,
    Dime,
    Quarter,
    Dollar,
}

fn value_in_cents(coin: Coin) -> u8 { // Assign values to each coin in the database
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Dollar => 100,
    }
}

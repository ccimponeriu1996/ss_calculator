use std::io;

fn main() {
    
    let mut data_vec: Vec<f64> = Vec::new();
    let mut value: f64;
    let mut count = 0;

    //Stores the number of balances being input.
    println!("Please input how many balances you would like to compare:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Line read failed.");
    let balances: u8 = input.trim().parse().expect("Not a number?");
  
    //Stores blaances in a vector
    println!("Please input the balances you would like to compare:");
    while count <  balances {
        input = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't read.");
        value = input.trim().parse().expect("Couldn't convert to float.");
        data_vec.push(value);
        count += 1;
    }
    
    //Stores target sum in variable from user input
    println!("What should be the target sum?");
    input = String::new();
    io::stdin().read_line(&mut input).expect("No read target sum.");
    let target: f64 = input.trim().parse().expect("Can't parse target.");

    //Stores number of values to compare
    println!("How many of these values would you like to compare for the sum?");
    input = String::new();
    io::stdin().read_line(&mut input).expect("No read target sum.");
    let mut target_combo: u8 = input.trim().parse().expect("Target not within range.");
    
    //Checks if target is within range
    while target_combo > balances {
        println!("Target cannot be greater than sample size!");
        input = String::new();
        io::stdin().read_line(&mut input).expect("No read target sum.");
        target_combo = input.trim().parse().expect("Target not within u8 range.");
    }
    let target_combo = target_combo;

    count = 0;
    while count < balances {
        println!("{}", count);
        count += 1;
    }
}

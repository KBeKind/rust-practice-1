
use std::io;


fn main() {

    let proceed: bool = true;

    if proceed {
        println!("proceeding");
    } else {
        println!("not proceeding");
    }

    let mut height: i32 = 190;
    let result: &str = if height > 180 {
            "Tall"
        } else if height > 170 {
            "Average"
        } else {
            "Short"
        };

    // VARIABLE SHADOWING
    // USED mut TO MAKE VARIABLE MUTABLE
    height = height - 20;

    println!("The result is: {}", result);


    let health: &str = if height < 180 {"good"} else {"unknown"};
    println!("The health is: {}", health);


let mut x = 1;

loop {
    println!("The value of x is: {}", x);
    x += 1;
    if x >= 5 {
        break;
    
    }
}


// There are other conditionals that we can explore in Rust. Like using `if let`


let maybe_number: Option<Option<()>> = None;
//let maybe_number = Some(42);
if let Some(number) = maybe_number {
    println!("The number is {:?}", number);
} else {
    println!("There is no number");
}



    // WHILE LOOPS

    let mut i = 0;
    while i < 5  {
        println!("The value of i is: {}", i);
        i += 1;
    }

    let mut input = String::new();

    while input.trim() != "quit" {
        input.clear();
        println!("Enter a word (type 'quit' to exit): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input);

    }
    println!("Exiting...");
    
}

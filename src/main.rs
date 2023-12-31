
use std::{io, result};


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

    let mut i: i32 = 0;
    while i < 5  {
        println!("The value of i is: {}", i);
        i += 1;
    }

    // let mut input = String::new();

    // while input.trim() != "quit" {
    //     input.clear();
    //     println!("Enter a word (type 'quit' to exit): ");
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     println!("You entered: {}", input);

    // }
    // println!("Exiting...");
    

    // FOR LOOPS

    // DOESNT INCLUDE 5
    for i in 0..5 {
        println!("The value of i is: {}", i);
    }

    // INCLUDES 5
    for i in 0..=5 {
        println!("The value of i is: {}", i);
    }

    // REVERSE THE ORDER WITH .rev
    for i in (0..5).rev() {
        println!("The value of i is: {}", i);
    }


    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("The value of n is: {}", n);
    }


    println!("continue and break test below");

    // USING CONTINUE AND BREAK
    for i in 1..=10 {
        if i % 2 == 0 {
            // SKIPS EVEN NUMBERS
            // CONTINUE TAKES US TO NEXT ROUND OF LOOP
            continue;
        }
        println!("The value of i is: {}", i);
        if i == 7 {
        break;
        }

    }



    // MATCH CONTROL FLOW

    let name: &str = "Hello";

    match name {
        "Hello" => println!("Hello"),
        "World" => println!("World"),
        // THE _ MEANS DEFAULT
        _ => println!("Default"),
    }


    // println!("Please enter a greeting:");
    // let mut greeting: String = String::new();
    // io::stdin().read_line(&mut greeting).expect("Failed to read line");
    // match greeting.trim() {
    //     "Hello" => println!("Hi Welcome"),
    //     "Goodbye" => println!("Bye Bye"),
    //     _ => println!("Default greeting because i dont know what you said"),
    
    // }


    // CALLING FUNCTIONS

    process_numbers(&[1,2,3]);


    let chunk: String = split_string("hello,world".to_string(), ',', 0);

    println!("The chunk is: {}", chunk);


    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result: i32 = sum(&numbers);
    println!("The sum is: {}", result);


}

// UNIT FUNCTIONS HAVE NO RETURN VALUE
fn process_numbers(numbers: &[i32]) {
    // Initialze the sum to zero
    let mut sum: i32 = 0;


    for number in numbers {

        sum += number;
        println!("The sum is: {}", sum);
    
    }

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }



}

// FUNCTION WITH A RETURN VALUE STRING
fn split_string(aString: String, delimeter: char, field: usize) -> String {

    let parts: Vec<&str> = aString.split(delimeter).collect();
    let result: Option<&&str> = parts.get(field);
    // USING THE OPTION WITH A PANIC THERE IN CASE IT DOESNT EXIST
    result.expect("oops: something wrong").to_string()
}

// FUNCTION WITH A RETURN VALUE INTEGER i32
fn sum(numbers: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for number in numbers {
        total += number;
    }
    total
}
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


}
